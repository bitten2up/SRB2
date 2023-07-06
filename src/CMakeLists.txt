add_executable(SRB2SDL2 MACOSX_BUNDLE WIN32)

if("${CMAKE_COMPILER_IS_GNUCC}" AND "${CMAKE_SYSTEM_NAME}" MATCHES "Windows" AND NOT "${SRB2_CONFIG_SYSTEM_LIBRARIES}" AND NOT "${SRB2_CONFIG_SHARED_INTERNAL_LIBRARIES}")
	# On MinGW with internal libraries, link the standard library statically
	target_link_options(SRB2SDL2 PRIVATE "-static")
endif()

# Core sources
target_sourcefile(c)
target_sources(SRB2SDL2 PRIVATE comptime.c md5.c config.h.in)

set(SRB2_ASM_SOURCES vid_copy.s)

set(SRB2_NASM_SOURCES tmap_mmx.nas tmap.nas)

### Configuration
set(SRB2_CONFIG_USEASM OFF CACHE BOOL
	"Enable NASM tmap implementation for software mode speedup.")
set(SRB2_CONFIG_YASM OFF CACHE BOOL
	"Use YASM in place of NASM.")
set(SRB2_CONFIG_DEV_BUILD OFF CACHE BOOL
	"Compile a development build of SRB2.")
set(SRB2_CONFIG_HAVE_PNG ON CACHE BOOL
	"Enable PNG support. Depends on zlib, so will be disabled if you don't enable that too.")
set(SRB2_CONFIG_HAVE_ZLIB ON CACHE BOOL
	"Enable zlib support.")
if(NOT ${CMAKE_SYSTEM} MATCHES Emscripten)
	set(SRB2_CONFIG_HAVE_GME ON CACHE BOOL
		"Enable GME support.")
	set(SRB2_CONFIG_HAVE_OPENMPT ON CACHE BOOL
		"Enable OpenMPT support.")
	set(SRB2_CONFIG_HAVE_CURL ON CACHE BOOL
		"Enable curl support.")
	set(SRB2_CONFIG_HAVE_THREADS ON CACHE BOOL
		"Enable multithreading support.")
	if(${CMAKE_SYSTEM} MATCHES Windows)
		set(SRB2_CONFIG_HAVE_MIXERX ON CACHE BOOL
			"Enable SDL Mixer X support.")
	else()
		set(SRB2_CONFIG_HAVE_MIXERX OFF)
	endif()
	set(SRB2_CONFIG_HWRENDER ON CACHE BOOL
		"Enable hardware rendering through OpenGL.")
	set(SRB2_CONFIG_USEASM OFF CACHE BOOL
		"Enable NASM tmap implementation for software mode speedup.")
	set(SRB2_CONFIG_YASM OFF CACHE BOOL
		"Use YASM in place of NASM.")
	set(SRB2_CONFIG_STATIC_OPENGL OFF CACHE BOOL
		"Use statically linked OpenGL. NOT RECOMMENDED.")
endif()

add_subdirectory(blua)

# OS macros
if (UNIX)
	target_compile_definitions(SRB2SDL2 PRIVATE -DUNIXCOMMON)
endif()

if(CMAKE_COMPILER_IS_GNUCC)
	find_program(OBJCOPY objcopy)
endif()

if("${CMAKE_SYSTEM_NAME}" MATCHES "Linux")
	target_compile_definitions(SRB2SDL2 PRIVATE -DLINUX)
	if(${SRB2_SYSTEM_BITS} EQUAL 64)
		target_compile_definitions(SRB2SDL2 PRIVATE -DLINUX64)
	endif()
endif()

if("${CMAKE_SYSTEM_NAME}" MATCHES "Darwin")
	target_compile_definitions(SRB2SDL2 PRIVATE -DMACOSX)
endif()

target_link_libraries(SRB2SDL2 PRIVATE gme)
target_compile_definitions(SRB2SDL2 PRIVATE -DHAVE_GME)
if(NOT "${SRB2_CONFIG_SYSTEM_LIBRARIES}")
	# this sucks but gme doesn't use modern cmake to delineate public headers
	target_include_directories(SRB2SDL2 PRIVATE "${libgme_SOURCE_DIR}")
endif()

target_link_libraries(SRB2SDL2 PRIVATE openmpt)
target_compile_definitions(SRB2SDL2 PRIVATE -DHAVE_OPENMPT)

target_link_libraries(SRB2SDL2 PRIVATE ZLIB::ZLIB PNG::PNG CURL::libcurl)
target_compile_definitions(SRB2SDL2 PRIVATE -DHAVE_ZLIB -DHAVE_PNG -DHAVE_CURL -D_LARGEFILE64_SOURCE)
target_sources(SRB2SDL2 PRIVATE apng.c)

set(SRB2_HAVE_THREADS ON)
target_compile_definitions(SRB2SDL2 PRIVATE -DHAVE_THREADS)

if(${SRB2_CONFIG_HAVE_ZLIB})
	if(${SRB2_CONFIG_USE_INTERNAL_LIBRARIES})
		set(ZLIB_FOUND ON)
		set(ZLIB_INCLUDE_DIRS ${CMAKE_SOURCE_DIR}/libs/zlib)
		if(${SRB2_SYSTEM_BITS} EQUAL 64)
			set(ZLIB_LIBRARIES "-L${CMAKE_SOURCE_DIR}/libs/zlib/win32 -lz64")
		else() # 32-bit
			set(ZLIB_LIBRARIES "-L${CMAKE_SOURCE_DIR}/libs/zlib/win32 -lz32")
		endif()
	elseif(${CMAKE_SYSTEM} MATCHES Emscripten)
		# \todo handle Emscripten library switches here, not in emscripten/CMakeLists.txt
		set(ZLIB_FOUND ON)
	else()
		find_package(ZLIB)
	endif()
	if(${ZLIB_FOUND})
		set(SRB2_HAVE_ZLIB ON)
		add_definitions(-DHAVE_ZLIB)
	else()
		message(WARNING "You have specified that ZLIB is available but it was not found. SRB2 may not compile correctly.")
	endif()
endif()

if(${SRB2_CONFIG_HAVE_PNG} AND ${SRB2_CONFIG_HAVE_ZLIB})
	if (${ZLIB_FOUND})
		if(${SRB2_CONFIG_USE_INTERNAL_LIBRARIES})
			set(PNG_FOUND ON)
			set(PNG_INCLUDE_DIRS ${CMAKE_SOURCE_DIR}/libs/libpng-src)
			if(${SRB2_SYSTEM_BITS} EQUAL 64)
				set(PNG_LIBRARIES "-L${CMAKE_SOURCE_DIR}/libs/libpng-src/projects -lpng64")
			else() # 32-bit
				set(PNG_LIBRARIES "-L${CMAKE_SOURCE_DIR}/libs/libpng-src/projects -lpng32")
			endif()
		elseif(${CMAKE_SYSTEM} MATCHES Emscripten)
			# \todo handle Emscripten library switches here, not in emscripten/CMakeLists.txt
			set(PNG_FOUND ON)
		else()
			message(WARNING "You have specified static opengl but opengl was not found. Not setting HWRENDER.")
		endif()
	endif()
endif()

if(${SRB2_CONFIG_USEASM})
	#SRB2_ASM_FLAGS can be used to pass flags to either nasm or yasm.
	if("${CMAKE_SYSTEM_NAME}" MATCHES "Linux")
		set(SRB2_ASM_FLAGS "-DLINUX ${SRB2_ASM_FLAGS}")
	endif()

	if(${SRB2_CONFIG_YASM})
		set(CMAKE_ASM_YASM_SOURCE_FILE_EXTENSIONS ${CMAKE_ASM_YASM_SOURCE_FILE_EXTENSIONS} nas)
		set(CMAKE_ASM_YASM_FLAGS "${SRB2_ASM_FLAGS}" CACHE STRING "Flags used by the assembler during all build types.")
		enable_language(ASM_YASM)
	else()
		set(CMAKE_ASM_NASM_SOURCE_FILE_EXTENSIONS ${CMAKE_ASM_NASM_SOURCE_FILE_EXTENSIONS} nas)
		set(CMAKE_ASM_NASM_FLAGS "${SRB2_ASM_FLAGS}" CACHE STRING "Flags used by the assembler during all build types.")
		enable_language(ASM_NASM)
	endif()

	set(SRB2_USEASM ON)
	target_compile_definitions(SRB2SDL2 PRIVATE -DUSEASM)
	target_compile_options(SRB2SDL2 PRIVATE -msse3 -mfpmath=sse)

	target_sources(SRB2SDL2 PRIVATE ${SRB2_ASM_SOURCES}
		${SRB2_NASM_SOURCES})
else()
	set(SRB2_USEASM OFF)
	target_compile_definitions(SRB2SDL2 PRIVATE -DNONX86 -DNORUSEASM)
endif()

# Targets

# If using CCACHE, then force it.
# https://github.com/Cockatrice/Cockatrice/pull/3052/files
if ("${CMAKE_SYSTEM_NAME}" MATCHES "Darwin")
	get_property(RULE_LAUNCH_COMPILE GLOBAL PROPERTY RULE_LAUNCH_COMPILE)
	if(RULE_LAUNCH_COMPILE)
		MESSAGE(STATUS "Force enabling CCache usage under macOS")
		# Set up wrapper scripts
		configure_file(${CMAKE_CURRENT_SOURCE_DIR}/../cmake/launch-c.in   ${CMAKE_BINARY_DIR}/launch-c)
		configure_file(${CMAKE_CURRENT_SOURCE_DIR}/../cmake/launch-cxx.in ${CMAKE_BINARY_DIR}/launch-cxx)
		execute_process(COMMAND chmod a+rx
			"${CMAKE_BINARY_DIR}/launch-c"
			"${CMAKE_BINARY_DIR}/launch-cxx")

		# Set Xcode project attributes to route compilation through our scripts
		set(CMAKE_XCODE_ATTRIBUTE_CC         "${CMAKE_BINARY_DIR}/launch-c")
		set(CMAKE_XCODE_ATTRIBUTE_CXX        "${CMAKE_BINARY_DIR}/launch-cxx")
		set(CMAKE_XCODE_ATTRIBUTE_LD         "${CMAKE_BINARY_DIR}/launch-c")
		set(CMAKE_XCODE_ATTRIBUTE_LDPLUSPLUS "${CMAKE_BINARY_DIR}/launch-cxx")
	endif()
endif()

# Compatibility flag with later versions of GCC
# We should really fix our code to not need this
if ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU")
	if (CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|x64|amd64|AMD64|em64t|EM64T)")
		target_compile_options(SRB2SDL2 PRIVATE -mno-ms-bitfields)
	endif()
endif()

# Compiler warnings configuration
target_compile_options(SRB2SDL2 PRIVATE
	# Using generator expressions to handle per-language compile options

	# C, GNU
	# This is a direct translation from versions.mk
	$<$<AND:$<COMPILE_LANGUAGE:C>,$<C_COMPILER_ID:GNU>>:
		-Wall
		-Wno-trigraphs
		-W # Was controlled by RELAXWARNINGS
		-Wfloat-equal
		-Wundef
		-Wpointer-arith
		-Wbad-function-cast
		-Wcast-qual
		-Wcast-align # Was controlled by NOCASTALIGNWARN
		-Wwrite-strings
		-Wsign-compare
		-Waggregate-return
		-Wmissing-prototypes
		-Wmissing-declarations
		-Wmissing-noreturn
		-Wnested-externs
		-Winline
		-Wformat-y2k
		-Wformat-security

		$<$<VERSION_LESS:$<C_COMPILER_VERSION>,2.9.5>:
			-Wno-div-by-zero
			-Wendif-labels
			-Wdisabled-optimization
		>

		$<$<VERSION_GREATER_EQUAL:$<C_COMPILER_VERSION>,4.0.0>:
			-Wold-style-definition
			-Wmissing-field-initializers
		>

		$<$<VERSION_GREATER_EQUAL:$<C_COMPILER_VERSION>,4.1.0>:
			-Wshadow
		>

		$<$<VERSION_GREATER_EQUAL:$<C_COMPILER_VERSION>,4.3.0>:
			-funit-at-a-time
			-Wlogical-op
		>

		$<$<VERSION_GREATER_EQUAL:$<C_COMPILER_VERSION>,4.5.0>:
			-Wlogical-op
			-Wno-error=array-bounds
		>

		$<$<VERSION_GREATER_EQUAL:$<C_COMPILER_VERSION>,4.6.0>:
			-Wno-suggest-attribute=noreturn
			-Wno-error=suggest-attribute=noreturn
		>

		$<$<VERSION_GREATER_EQUAL:$<C_COMPILER_VERSION>,5.4.0>:
			-Wno-logical-op
			-Wno-error=logical-op
		>

		$<$<VERSION_GREATER_EQUAL:$<C_COMPILER_VERSION>,6.1.0>:
			-Wno-tautological-compare
			-Wno-error=tautological-compare
		>

		$<$<VERSION_GREATER_EQUAL:$<C_COMPILER_VERSION>,7.1.0>:
			-Wno-error=format-overflow=2
			-Wimplicit-fallthrough=4
		>

		$<$<VERSION_GREATER_EQUAL:$<C_COMPILER_VERSION>,8.1.0>:
			-Wno-error=format-overflow
			-Wno-error=stringop-truncation
			-Wno-error=stringop-overflow
			-Wno-format-overflow
			-Wno-stringop-truncation
			-Wno-stringop-overflow
			-Wno-error=multistatement-macros
		>

		$<$<VERSION_GREATER_EQUAL:$<C_COMPILER_VERSION>,9.1.0>:
			-Wno-error=address-of-packed-member
		>
	>

	# C, Clang and Apple Clang
	$<$<AND:$<COMPILE_LANGUAGE:C>,$<OR:$<C_COMPILER_ID:AppleClang>,$<C_COMPILER_ID:Clang>>>:
		-Wall
		-Wno-absolute-value
		-Wno-trigraphs
		-Wno-error=non-literal-null-conversion
		-Wno-error=constant-conversion
		-Wno-error=unused-but-set-variable
	>

	# C, MSVC
	$<$<AND:$<COMPILE_LANGUAGE:C>,$<C_COMPILER_ID:MSVC>>:
		# All warnings at and before Visual Studio 2019 RTM
		# https://learn.microsoft.com/en-us/cpp/error-messages/compiler-warnings/compiler-warnings-by-compiler-version?view=msvc-170
		/Wv:19.20.27004.0
	>

	# C++, GNU, Clang and Apple Clang
	$<$<AND:$<COMPILE_LANGUAGE:CXX>,$<OR:$<C_COMPILER_ID:GNU>,$<C_COMPILER_ID:AppleClang>,$<C_COMPILER_ID:Clang>>>:
		-Wall
	>

	# C++, MSVC
	$<$<AND:$<COMPILE_LANGUAGE:C>,$<C_COMPILER_ID:MSVC>>:
		/Wv:19.20.27004.0
	>
)
if(SRB2_CONFIG_ERRORMODE)
	target_compile_options(SRB2SDL2 PRIVATE
		$<$<OR:$<C_COMPILER_ID:GNU>,$<C_COMPILER_ID:AppleClang>,$<C_COMPILER_ID:Clang>>:
			-Werror
		>

		$<$<C_COMPILER_ID:MSVC>:
			/WX
		>
	)
endif()

# Link warnings configuration
target_link_options(SRB2SDL2 PRIVATE
	$<$<C_COMPILER_ID:GNU>:
		# -Wl,--as-needed   - Was controlled by NOLDWARNING
	>
)

if(${SRB2_CONFIG_DEV_BUILD})
	target_compile_definitions(SRB2SDL2 PRIVATE -DDEVELOP)
endif()
target_compile_definitions(SRB2SDL2 PRIVATE -DCMAKECONFIG)

# Misc. build options from Makefiles
if(SRB2_CONFIG_DEBUGMODE)
	target_compile_definitions(SRB2SDL2 PRIVATE -DZDEBUG -DPARANOIA -DRANGECHECK -DPACKETDROP)
endif()
if(SRB2_CONFIG_MOBJCONSISTANCY)
	target_compile_definitions(SRB2SDL2 PRIVATE -DMOBJCONSISTANCY)
endif()
if(SRB2_CONFIG_PACKETDROP)
	target_compile_definitions(SRB2SDL2 PRIVATE -DPACKETDROP)
endif()
if(SRB2_CONFIG_ZDEBUG)
	target_compile_definitions(SRB2SDL2 PRIVATE -DZDEBUG)
endif()
if(SRB2_CONFIG_PROFILEMODE AND "${CMAKE_C_COMPILER_ID}" STREQUAL "GNU")
	target_compile_options(SRB2SDL2 PRIVATE -pg)
	target_link_options(SRB2SDL2 PRIVATE -pg)
endif()

add_subdirectory(sdl)

# strip debug symbols into separate file when using gcc.
# to be consistent with Makefile, don't generate for OS X.
if((CMAKE_COMPILER_IS_GNUCC) AND NOT ("${CMAKE_SYSTEM_NAME}" MATCHES Darwin))
	if((${CMAKE_BUILD_TYPE} MATCHES Debug) OR (${CMAKE_BUILD_TYPE} MATCHES RelWithDebInfo))
		if(${CMAKE_BUILD_TYPE} MATCHES Debug)
			set(OBJCOPY_ONLY_KEEP_DEBUG "--only-keep-debug")
		endif()
		message(STATUS "Will make separate debug symbols in *.debug")
		add_custom_command(TARGET SRB2SDL2 POST_BUILD
			COMMAND ${OBJCOPY} ${OBJCOPY_ONLY_KEEP_DEBUG} $<TARGET_FILE:SRB2SDL2> $<TARGET_FILE:SRB2SDL2>.debug
			COMMAND ${OBJCOPY} --strip-debug $<TARGET_FILE:SRB2SDL2>
			COMMAND ${OBJCOPY} --add-gnu-debuglink=$<TARGET_FILE:SRB2SDL2>.debug $<TARGET_FILE:SRB2SDL2>
		)
	endif()
endif()

# copy DLLs to bin/ directory if building internal shared on windows
if("${CMAKE_SYSTEM_NAME}" STREQUAL Windows AND NOT "${SRB2_CONFIG_INTERNAL_LIBRARIES}" AND "${SRB2_CONFIG_SHARED_INTERNAL_LIBRARIES}")
	set(ADDITIONAL_DLLS "")
	if("${CMAKE_C_COMPILER_ID}" STREQUAL GNU)
		# also copy implicitly linked system libraries
		get_filename_component(MINGW_BIN_PATH ${CMAKE_CXX_COMPILER} PATH)
		list(APPEND ADDITIONAL_DLLS
			"libgcc_s_dw2-1.dll"
			"libstdc++-6.dll"
			"libwinpthread-1.dll"
		)
		list(TRANSFORM ADDITIONAL_DLLS PREPEND "${MINGW_BIN_PATH}/")
	endif()
	add_custom_command(TARGET SRB2SDL2 POST_BUILD
		COMMAND ${CMAKE_COMMAND} -E copy_if_different
			$<TARGET_RUNTIME_DLLS:SRB2SDL2>
			${ADDITIONAL_DLLS}

			$<TARGET_FILE_DIR:SRB2SDL2>
		COMMAND_EXPAND_LISTS
		COMMENT "Copying runtime DLLs"
	)
endif()
