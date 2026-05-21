add_executable(mapvina-slint-example
    main.cpp
    src/slint_mapvina_headless.cpp
    platform/custom_file_source.cpp
)

if (WIN32)
  target_compile_definitions(mapvina-slint-example
    PRIVATE NOMINMAX _USE_MATH_DEFINES WIN32_LEAN_AND_MEAN)

  if (MSVC)
    target_compile_options(mapvina-slint-example PRIVATE /utf-8 /MP)
  endif()

  find_package(libuv CONFIG REQUIRED)

  if (TARGET libuv::uv_a)
    set(ML_UV_TARGET libuv::uv_a)
  else()
    set(ML_UV_TARGET libuv::uv)
  endif()

  target_link_libraries(mapvina-slint-example PRIVATE ${ML_UV_TARGET})
endif()

# Associate .slint files with the target (only if the helper is available)
if (COMMAND slint_target_sources)
    slint_target_sources(mapvina-slint-example
        ${CMAKE_CURRENT_SOURCE_DIR}/map_window.slint
    )
else()
    message(FATAL_ERROR "slint_target_sources command is not available. Ensure Slint's CMake integration is present.")
endif()

# Include dirs for this project (Slint::Slint will propagate its own include dirs)
target_include_directories(mapvina-slint-example
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/src
        ${CMAKE_CURRENT_SOURCE_DIR}/platform
        ${CMAKE_SOURCE_DIR}/vendor/mapvina-native/include
        ${CMAKE_BINARY_DIR}/vendor/mapvina-native/include
        ${CMAKE_CURRENT_BINARY_DIR}
        $<$<NOT:$<BOOL:${MLN_WITH_WEBGPU}>>:${GLES3_INCLUDE_DIRS}>
)

# Link libraries
target_link_libraries(mapvina-slint-example
    PRIVATE
        Slint::Slint
        mbgl-core
        cpr::cpr
)

if(MLN_WITH_WEBGPU)
  target_compile_definitions(mapvina-slint-example PRIVATE MLN_WITH_WEBGPU)
  # WebGPU backend: link wgpu-native via the mbgl-vendor-wgpu target from mapvina-native
  if(TARGET mbgl-vendor-wgpu)
    target_link_libraries(mapvina-slint-example PRIVATE mbgl-vendor-wgpu)
  endif()
else()
  # Traditional OpenGL/Metal backend
  target_link_libraries(mapvina-slint-example
      PRIVATE
          ${GLES3_LIBRARIES}
          ${OPENGL_LIBRARIES}
          $<$<PLATFORM_ID:Darwin>:${METAL_FRAMEWORK}>
  )
endif()

# On Windows, copy the Slint DLL next to the application binary so that it's found
if (WIN32)
    add_custom_command(TARGET mapvina-slint-example POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_RUNTIME_DLLS:mapvina-slint-example> $<TARGET_FILE_DIR:mapvina-slint-example>
        COMMAND_EXPAND_LISTS)

    # Copy wgpu_native.dll when using the WebGPU backend
    if(MLN_WITH_WEBGPU AND DEFINED WGPU_LIBRARY)
        string(REPLACE ".lib" ".dll" _wgpu_dll "${WGPU_LIBRARY}")
        if(EXISTS "${_wgpu_dll}")
            add_custom_command(TARGET mapvina-slint-example POST_BUILD
                COMMAND ${CMAKE_COMMAND} -E copy_if_different
                    "${_wgpu_dll}"
                    "$<TARGET_FILE_DIR:mapvina-slint-example>"
                COMMENT "Copying wgpu_native.dll to output directory"
            )
        endif()
    endif()
endif()
