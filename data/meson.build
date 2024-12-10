
install_data(
  'com.github.flxzt.rnote.xml',
  install_dir: datadir / 'mime' / 'packages'
)

install_data(
  'rnote.thumbnailer',
  install_dir: datadir / 'thumbnailers'
)

gnome.post_install(
  update_mime_database: true
)
