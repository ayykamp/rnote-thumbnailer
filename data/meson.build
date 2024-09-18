
install_data(
  'raw-thumbnailer.xml',
  install_dir: join_paths(datadir, 'mime', 'packages')
)

install_data(
  'raw.thumbnailer',
  install_dir: join_paths(datadir, 'thumbnailers')
)

gnome.post_install(
  update_mime_database: true
)
