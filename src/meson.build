inc_src = include_directories('../src')

gtk_dep = dependency('gtk4', required : true)
adw_dep = dependency('libadwaita-1', required : true)
rsvg = dependency('librsvg-2.0', required: true)

cargo = find_program('cargo', required: true)

install_data('../data/metainfo/io.github.rsvzz.iconvwadw.metainfo.xml', install_dir: get_option('prefix') / 'share/metainfo')
install_data('../data/icons/io.github.rsvzz.iconvwadw.svg', install_dir: get_option('prefix') / 'share/icons/hicolor/scalable/apps')
install_data('../data/desktop/io.github.rsvzz.iconvwadw.desktop', install_dir: get_option('prefix') /'share/applications')

icons_file = files(
   '../data/ui/main.ui',
   '../data/ui/about.ui',
   '../data/ui/view_data.ui'
)
install_data(icons_file, install_dir: get_option('prefix') / 'share/iconvwadw/ui')

buildtype = get_option('buildtype')
cargo_flag = []
src_exe = 'debug'

if buildtype == 'release' or buildtype == 'debugoptimized'
  cargo_flag = ['--release']
  src_exe = 'release'
endif

cargo_build = custom_target('cargo-build',
  output: 'iconvwadw-build',
  build_by_default: true,
  env:['CARGO_HOME=' + meson.project_build_root() / 'cargo'],
  command: [
    'cargo', 'build'] + cargo_flag + [
    '--manifest-path', meson.project_source_root() / 'Cargo.toml',
    '--target-dir', meson.project_build_root() / 'src'
  ],
  build_always_stale: true
)

exe_name = 'iconvwadw'
if host_machine.system() == 'windows'
  exe_name += '.exe'
endif

custom_target('iconvwadw',
  output: exe_name,
  build_by_default: true,
  build_always_stale: true,
  depends: cargo_build,
  env:['CARGO_HOME=' + meson.project_build_root() / 'cargo'],
  install: true,
  install_dir : get_option('bindir'),
  command: ['cp', meson.project_build_root() / 'src/' + src_exe + '/' + exe_name, '@OUTPUT@']
)