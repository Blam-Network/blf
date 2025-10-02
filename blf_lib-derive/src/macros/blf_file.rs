use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn blf_file_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name: syn::Ident = input.ident.clone();

    match input.data {
        Data::Struct(body) => {
            let writes = body.fields.iter().map(|field| {
                let field_name = format_ident!("{}", field.clone().ident.unwrap().to_string());

                quote! {
                    data.append(&mut blf_lib::blf::chunks::SerializableBlfChunk::write(&mut self.#field_name, &data)?);
                }
            });

            let reads = body.fields.iter().map(|field| {
                let field_name = format_ident!("{}", field.clone().ident.unwrap().to_string());

                quote! {
                    reader.read_exact(&mut header_bytes)?;
                    header = blf_lib::blf::s_blf_header::decode(&header_bytes)?;

                    if header.signature == blf_lib::blf::chunks::DynamicBlfChunk::signature(&blf_file.#field_name) && header.version == blf_lib::blf::chunks::DynamicBlfChunk::version(&blf_file.#field_name) {
                        let mut body_bytes = vec![0u8; (header.chunk_size as usize) - blf_lib::blf::s_blf_header::size()];
                        reader.read_exact(body_bytes.as_mut_slice())?;
                        blf_lib::blf::chunks::SerializableBlfChunk::decode_body(&mut blf_file.#field_name, body_bytes.as_slice(), &previously_read)?;

                        previously_read.extend_from_slice(&header_bytes);
                        previously_read.extend_from_slice(&body_bytes);
                    }
                    else {
                        return Err(format!("{} Chunk not found!", blf_lib::blf::chunks::DynamicBlfChunk::signature(&blf_file.#field_name).to_string()).into());
                    }
                }
            });


            (quote! {
                impl blf_lib::blf::BlfFile for #name {
                    fn write(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
                        let mut data: Vec<u8> = Vec::new();
                        #(#writes)*

                        Ok(data)
                    }

                    fn write_file(&mut self, path: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
                        let path = &path.into();
                        let mut data = Self::write(self)?;

                        let parent_path = std::path::Path::new(path).parent().ok_or(std::fmt::Error)?;
                        std::fs::create_dir_all(parent_path)?;

                        std::io::Write::write_all(&mut std::fs::File::create(path)?, &data)
                            .map_err(|e| e.into())
                    }

                    fn read_file(path: &String) -> Result<Self, Box<dyn std::error::Error>> {
                        let mut reader = std::fs::File::open(path)?;

                        Self::read(&mut reader)
                    }

                    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {
                        let mut header_bytes = [0u8; blf_lib::blf::s_blf_header::size()];
                        let mut header: blf_lib::blf::s_blf_header;
                        let mut previously_read = Vec::<u8>::new();

                        let mut blf_file = Self::default();

                        #(#reads)*

                        Ok(blf_file)
                    }
                }
            }).into()
        }
        _ => panic!("Tried to apply BlfFile derive to a non-struct!")
    }
}