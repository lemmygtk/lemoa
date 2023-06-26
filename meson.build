project(
  'lemoa',
  'rust',
  version: '0.1.0',
  meson_version: '>= 0.59',
  license: 'GPL-3.0',
)

gnome = import('gnome')

base_id = 'com.lemmygtk.lemoa'
version = meson.project_version()

# Required packages
dependency('glib-2.0', version: '>= 2.70')
dependency('gio-2.0', version: '>= 2.70')
dependency('gtk4', version: '>= 4.10.0')

glib_compile_resources = find_program('glib-compile-resources', required: true)
glib_compile_schemas = find_program('glib-compile-schemas', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)
cargo = find_program('cargo', required: true)

# Various directories needed for building
prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')
iconsdir = datadir / 'icons'
applicationsdir = datadir / 'applications'
pkgdatadir = datadir / meson.project_name()

# Add a version suffix for devel builds
if get_option('profile') == 'development'
  profile = 'Devel'
  vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD', check: false).stdout().strip()
  if vcs_tag == ''
    version_suffix = '-devel'
  else
    version_suffix = '-@0@'.format(vcs_tag)
  endif
  application_id = '@0@.@1@'.format(base_id, profile)
else
  profile = ''
  version_suffix = ''
  application_id = base_id
endif

# Modules that need to be built
subdir('src')
subdir('data')

gnome.post_install(
  gtk_update_icon_cache: true,
  glib_compile_schemas: true,
  update_desktop_database: true,
)
