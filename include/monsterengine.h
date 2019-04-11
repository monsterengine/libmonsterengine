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
  void monster_engine_server_start(PlamoApp *app, MonsterEngineConfig *config);

#ifdef __cplusplus
}
#endif
