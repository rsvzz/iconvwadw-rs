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

custom_target('iconvwadw',
  output: 'iconvwadw',
  build_by_default: true,
  env:['CARGO_HOME=' + meson.project_build_root() / 'cargo'],
  install : true,
  install_dir : get_option('bindir'),
  command: ['cargo', 'build', '--release', '--manifest-path', meson.project_source_root() + '/Cargo.toml'
  ,'--target-dir', meson.project_build_root() / 'src',
   '&&', 'cp', meson.project_build_root() / 'src/release/iconvwadw', '@OUTPUT@']
)