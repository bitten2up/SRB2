include(LibFindMacros)

libfind_pkg_check_modules(DISCORDGAMESDK_PKGCONF DISCORDGAMESDK)

find_path(DISCORDGAMESDK_INCLUDE_DIR
	NAMES discord_game_sdk.h
	PATHS
		${DISCORDGAMESDK_PKGCONF_INCLUDE_DIRS}
		"/usr/include"
		"/usr/local/include"
)

find_library(DISCORDGAMESDK_LIBRARY
	NAMES discord_game_sdk
	PATHS
		${DISCORDGAMESDK_PKGCONF_LIBRARY_DIRS}
		"/usr/lib"
		"/usr/local/lib"
)

set(DISCORDGAMESDK_PROCESS_INCLUDES DISCORDGAMESDK_INCLUDE_DIR)
set(DISCORDGAMESDK_PROCESS_LIBS DISCORDGAMESDK_LIBRARY)

libfind_process(DISCORDGAMESDK)
