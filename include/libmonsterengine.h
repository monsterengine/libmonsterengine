#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <libplamo.h>

#ifdef __cplusplus
extern "C" {
#endif

  typedef struct MonsterEngineConfig MonsterEngineConfig;
  MonsterEngineConfig* monster_engine_config_new();
  void monster_engine_config_destroy(MonsterEngineConfig *monster_engine_config);
  const char* monster_engine_config_get_bind(const MonsterEngineConfig *monster_engine_config);
  void monster_engine_config_set_bind(MonsterEngineConfig *monster_engine_config, const char *bind);
  typedef struct MonsterEngineServer MonsterEngineServer;
  MonsterEngineServer* monster_engine_server_new(PlamoApp *app, MonsterEngineConfig *config);
  void monster_engine_server_destroy(MonsterEngineServer *monster_engine_server);
  void monster_engine_server_start(const MonsterEngineServer *monster_engine_server);

#ifdef __cplusplus
}
#endif
