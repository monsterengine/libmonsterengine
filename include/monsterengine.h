#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <plamo.h>

#ifdef __cplusplus
extern "C" {
#endif

  typedef struct MonsterEngineConfig MonsterEngineConfig;
  MonsterEngineConfig* monster_engine_config_new(char *bind, unsigned int workers);
  void monster_engine_config_destroy(MonsterEngineConfig *monster_engine_config);

  typedef struct MonsterEngineServer MonsterEngineServer;
  MonsterEngineServer* monster_engine_server_new(const PlamoApp *app, const MonsterEngineConfig *config);
  void monster_engine_server_start(const MonsterEngineServer *monster_engine_server);

#ifdef __cplusplus
}
#endif
