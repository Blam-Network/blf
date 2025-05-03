# # this is dumb
# # cargo install cargo-version-util
# # cargo install cargo-edit
# set current_version=cargo version-util get-version
# #cargo publish -p blf_lib-derivable
# echo %current_version%
import subprocess

print("blf_lib - publish script")
current_version = subprocess.check_output(['cargo', 'version-util', 'get-version']).decode().replace('\n', '').replace('\r', '')
print(f"current version = {current_version}")
print("please enter the new version")
new_version = input("new version = ")
subprocess.call(['cargo', 'version-util', 'set-version', new_version])

subprocess.call(['cargo', 'upgrade', '-p', f'blf_lib-derivable@{new_version}', '--manifest-path', 'blf_lib-derive/Cargo.toml', '--pinned'])
subprocess.call(['cargo', 'upgrade', '-p', f'blf_lib-derivable@{new_version}', '-p', f'blf_lib-derive@{new_version}', '--manifest-path', 'blf_lib/Cargo.toml', '--pinned'])
subprocess.call(['cargo', 'upgrade', '-p', f'blf_lib@{new_version}', '--manifest-path', 'blf_cli/Cargo.toml', '--pinned'])

subprocess.call(['npm', 'version', new_version], cwd='./blf_lsp-npm')
subprocess.call(['napi', 'version', '--config', '../blf_lsp/config.json'], cwd='./blf_lsp-npm')

subprocess.call(['git', 'add', 'Cargo.toml', '**/Cargo.toml', '**/package.json'])
subprocess.call(['git', 'commit', '-m', f'{new_version}'])
subprocess.call(['git', 'tag', new_version])
subprocess.call(['git', 'push', '--tags'])
