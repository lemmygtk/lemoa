desktop_conf = configuration_data()
desktop_conf.set('NAME', meson.project_name())
desktop_conf.set('ICON', application_id)
desktop_conf.set('APP_ID', application_id)
desktop_conf.set('BASE_ID', base_id)
desktop_conf.set('EXEC', meson.project_name())

# Desktop file
configure_file(
  input: '@0@.desktop.in'.format(base_id),
  output: '@BASENAME@',
  install: true,
  configuration: desktop_conf,
  install_dir: datadir / 'applications'
)

# Metainfo file
configure_file(
  input: '@0@.metainfo.xml.in'.format(base_id),
  output: '@BASENAME@',
  install: true,
  configuration: desktop_conf,
  install_dir:  datadir / 'metainfo'
)

configure_file(
  input: '@0@.gschema.xml.in'.format(base_id),
  output: '@0@.gschema.xml'.format(application_id),
  configuration: desktop_conf,
  install: true,
  install_dir: datadir / 'glib-2.0' / 'schemas'
)

subdir('icons')
