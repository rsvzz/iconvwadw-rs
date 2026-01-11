project('iconvwadw', 
'rust',
 version: '0.1.23'
 )
 
 meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + meson.project_version(),
  meson.project_source_root()
)

 subdir('src')