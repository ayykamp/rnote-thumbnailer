project(
  'tutorial', 'rust',
  version: '47'
)

cargo = find_program('cargo', required: true)
gnome = import('gnome')
i18n = import('i18n')

gettext_package = 'raw-thumbnailer'

datadir = get_option('datadir')

if get_option('profile') == 'development'
  profile = 'Devel'
  vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD', check: false).stdout().strip()
  if vcs_tag == ''
    version_suffix = '-devel'
  else
    version_suffix = '-@0@'.format(vcs_tag)
  endif
  package = '@0@-devel'.format(meson.project_name())
  app_name_suffix = ' (Development)'
else
  profile = ''
  version_suffix = ''
  package = meson.project_name()
  app_name_suffix = ''
endif

cargo_target_dir = meson.project_build_root() / 'target'
cargo_env = [
  'CARGO_HOME=' + meson.project_build_root() / 'cargo-home',
  'CARGO_TARGET_DIR=' + cargo_target_dir,
]
cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', cargo_target_dir ]

if get_option('profile') == 'release'
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif

gnome_raw_thumbnailer = custom_target(
  'cargo-build',
  build_always_stale: true,
  output: [
    'raw-thumbnailer'
  ],
  console: true,
  install: true,
  install_dir: 'bin',
  command: [
    'env',
    cargo_env,
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', cargo_target_dir / rust_target / 'raw-thumbnailer', '@OUTDIR@' / 'raw-thumbnailer'
  ],
)

subdir('data')
subdir('po')
