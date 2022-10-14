cmake_minimum_required(VERSION 3.15)

#
# Project name is SubconsciousShield.
#
# NOTE: If you change it make sure to change it in inf.in file. The sys file name
# will also be changed.
#
project(SubconsciousShield LANGUAGES CXX C VERSION 0.2.2)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

if (NOT PROJECT_VERSION)
    message(FATAL_ERROR "No PROJECT_VERSION is set.")
endif ()

#
# Compilation settings.
#
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /W4")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} /W4")


#
# The architecture returned from cmake is not the same as VS arch.
#
message(STATUS " [debug] CMAKE_HOST_SYSTEM_PROCESSOR=${CMAKE_HOST_SYSTEM_PROCESSOR}")
if ("${CMAKE_HOST_SYSTEM_PROCESSOR}" STREQUAL "AMD64")
    set(VS_ARCH x64)
else ()
    set(VS_ARCH x86)
endif ()

message(STATUS " [debug] vscode arch ${VS_ARCH}")

#
# Find required dependencies
#
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

find_package(WindowsSDK COMPONENTS tools REQUIRED)
find_package(WDK REQUIRED)

set(WINSDK ${WDK_ROOT}/bin/${WDK_VERSION}/x86)
message(STATUS "[Debug] WINSDK=${WINSDK}")

set(OPENSSL "openssl.exe")
set(MAKECERT "${WINSDK}/makecert.exe")
set(CERTUTIL "certutil.exe")
set(CERTMGR "certmgr.exe")
set(CERT2SPC "cert2spc.exe")
set(PVK2PFX "${WINSDK}/pvk2pfx.exe")
set(SIGNTOOL "${WINSDK}/signtool.exe")
set(INF2CAT "${WINSDK}/inf2cat.exe")

find_program(RUNDLL32 rundll32.exe REQUIRED)

# Windows SDK tools
find_program(INFVERIF infverif.exe
        PATHS ${WINDOWSSDK_DIRS}/Tools/${WDK_VERSION} ${WINDOWSSDK_DIRS}/Tools
        PATH_SUFFIXES x64 x86
        REQUIRED)

find_program(DEVCON devcon.exe
        PATHS ${WINDOWSSDK_DIRS}/Tools/${WDK_VERSION} ${WINDOWSSDK_DIRS}/Tools
        PATH_SUFFIXES x64 x86
        REQUIRED)

#
# Generate driver inf file and cat file.
#

# NOTE: driver version should match w.x.y.z where w,x,y,z ∈ (0, 2^31)
string(TIMESTAMP COMPILATION_TIMESTAMP UTC)
string(TIMESTAMP MMDDYY "%m/%d/%Y" UTC)

set(DRIVER_VERSION "${MMDDYY},${PROJECT_VERSION}.0")
message(STATUS "DRIVER_VERSION=${DRIVER_VERSION}")

set(INF_FILENAME FsFilter.inf)
set(INF_FILE ${CMAKE_CURRENT_BINARY_DIR}/${INF_FILENAME})

configure_file("${CMAKE_CURRENT_SOURCE_DIR}/${INF_FILENAME}.in" ${INF_FILE} @ONLY)

get_filename_component(CATALOG_FILENAME ${INF_FILENAME} NAME_WE)
set(CATALOG_FILE ${CMAKE_CURRENT_BINARY_DIR}/${CATALOG_FILENAME}.cat)

#
# Configuration for driver.
#
set(CONFIG_FILE_HEADER ${CMAKE_CURRENT_SOURCE_DIR}/FsFilter/config.h)
configure_file(FsFilter/config.h.in ${CONFIG_FILE_HEADER} @ONLY)
set_source_files_properties(${CONFIG_FILE_HEADER} PROPERTIES GENERATED TRUE)

#
# Compile driver.
#
wdk_add_driver(SubconsciousShield
        ${CONFIG_FILE_HEADER}
        FsFilter/Communication.cpp
        FsFilter/DriverData.cpp
        FsFilter/FSFilter.cpp
        FsFilter/FsFilter.rc
        FsFilter/KernelCommon.cpp
        FsFilter/KernelString.cpp
        FsFilter/ShanonEntropy.cpp
        SharedDefs/SharedDefs.h)

target_link_libraries(SubconsciousShield WDK::FLTMGR)

#
# set the properties on target.
# Using generator expression is really weired here but that how it seems to
# work in 2021 (https://stackoverflow.com/a/45871818/1805129)
set_target_properties(SubconsciousShield
        PROPERTIES
        LIBRARY_OUTPUT_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/$<0:>"
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/$<0:>")

#
# A target to check generated inf file. The check requires the
# SubconsciousShield.sys file has been generated. Hence the dependencies on
# SubconsciousShield.sys file (target SubconsciousSys)
add_custom_command(TARGET SubconsciousShield POST_BUILD
        COMMAND ${INFVERIF} ${INF_FILE}
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "\n\n=== Checking ${INF_FILE}")


#
# Generate certificate for self-signing.
# Thanks, https://github.com/SnoopWare/usbsnoop/blob/master/CMakeLists.txt
#
set(SUBCOM_CERT_NAME SubconsciousCert)
set(SUBCOM_CERT_OUTPUT_DIR "${CMAKE_CURRENT_SOURCE_DIR}/data")

set(SUBCOM_CERT_PATH "${SUBCOM_CERT_OUTPUT_DIR}/${SUBCOM_CERT_NAME}.pvk")
set(SUBCOM_PFX_FILE ${SUBCOM_CERT_OUTPUT_DIR}/${SUBCOM_CERT_NAME}.pfx)

if (NOT (EXISTS ${SUBCOM_PFX_FILE}))

    add_custom_command(OUTPUT ${SUBCOM_CERT_PATH} ${SUBCOM_PFX_FILE}
            COMMAND "${CMAKE_COMMAND}" -E remove ${SUBCOM_CERT_PATH} ${SUBCOM_CERT_NAME}.cer ${SUBCOM_CERT_NAME}.pfx ${SUBCOM_CERT_NAME}.spc
            COMMAND "${MAKECERT}" -b ${MMDDYY} -r -n \"CN=SubconsciousCompute\" -sv ${SUBCOM_CERT_PATH} ${SUBCOM_CERT_NAME}.cer
            COMMAND "${CERTMGR}" -add ${SUBCOM_CERT_NAME}.cer -s -r localMachine ROOT
            COMMAND "${CERTMGR}" -add ${SUBCOM_CERT_NAME}.cer -s -r localMachine TRUSTEDPUBLISHER
            COMMAND "${CERT2SPC}" ${SUBCOM_CERT_NAME}.cer ${SUBCOM_CERT_NAME}.spc
            COMMAND "${PVK2PFX}" -pvk ${SUBCOM_CERT_PATH} -spc ${SUBCOM_CERT_NAME}.spc -pfx ${SUBCOM_CERT_NAME}.pfx
            WORKING_DIRECTORY "${SUBCOM_CERT_OUTPUT_DIR}"
            COMMENT "Generating SSL certificates to sign the drivers and executable ...")

    add_custom_target(${SUBCOM_CERT_NAME} DEPENDS ${SUBCOM_CERT_PATH})
    add_dependencies(SubconsciousShield ${SUBCOM_CERT_NAME})
endif ()


#
# sign
# See the list of ts
# https://gist.github.com/Manouchehri/fd754e402d98430243455713efada710
#
set(SIGN_USING_SELF_SIGNED_CERT ON)
if (SIGN_USING_SELF_SIGNED_CERT)
    add_custom_command(TARGET SubconsciousShield POST_BUILD
            COMMAND "${SIGNTOOL}" sign /v /fd SHA256 /f ${SUBCOM_PFX_FILE}
            /t http://timestamp.digicert.com
            $<TARGET_FILE:SubconsciousShield>
            WORKING_DIRECTORY $<TARGET_FILE_DIR:SubconsciousShield>
            COMMENT "Signing SubconsciousShield ...")
else ()
    add_custom_command(TARGET SubconsciousShield POST_BUILD
            COMMAND "${SIGNTOOL}" sign /v /fd SHA256 $<TARGET_FILE:SubconsciousShield>
            WORKING_DIRECTORY $<TARGET_FILE_DIR:SubconsciousShield>
            COMMENT "Signing SubconsciousShield ...")
endif ()

#
# Generate catalog file from inf file. This target
#
add_custom_command(TARGET SubconsciousShield POST_BUILD
        COMMAND ${INF2CAT} /uselocaltime /driver:"${CMAKE_CURRENT_BINARY_DIR}" /os:10_RS5_X64
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "Generating catalog file from ${INF_FILE}")


#
# create scripts for installation and uninstallation of minifilter.
#
configure_file(${CMAKE_CURRENT_SOURCE_DIR}/dev/uninstall_minifilter.ps1.in
        ${CMAKE_CURRENT_BINARY_DIR}/uninstall_minifilter.ps1 @ONLY)

configure_file(${CMAKE_CURRENT_SOURCE_DIR}/dev/install_minifilter.ps1.in
        ${CMAKE_CURRENT_BINARY_DIR}/install_minifilter.ps1 @ONLY)

#
# Helper target to deploy the driver on development machine.
#
add_custom_target(install_minifilter
        DEPENDS SubconsciousShield
        COMMAND powershell.exe -ExecutionPolicy ByPass -File
        "${CMAKE_CURRENT_BINARY_DIR}/install_minifilter.ps1"
        COMMAND powershell.exe -ExecutionPolicy ByPass -File
        ${CMAKE_CURRENT_SOURCE_DIR}/data/hash_of_installed_and_generated_sysfile.ps1
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "Installing MiniFilter driver using ${INF_FILE}")

add_custom_target(uninstall_minifilter
        COMMAND powershell.exe -ExecutionPolicy ByPass -File ${CMAKE_CURRENT_BINARY_DIR}/uninstall_minifilter.ps1
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "Uninstalling MiniFilter driver.")

add_custom_target(deploy
        DEPENDS uninstall_minifilter install_minifilter
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "Uninstalling driver/Installing driver")


find_program(MAKENSIS_COMMAND makensis.exe
        PATHS ENV PROGRAMFILES64 /mingw64/bin /mingw32/bin
        PATH_SUFFIXES NSIS
        REQUIRED)


#
# PACKAGING
#

set(PACKAGE_NAME "${PROJECT_NAME}-${PROJECT_VERSION}-${CMAKE_HOST_SYSTEM_PROCESSOR}")
set(PACKAGE_OUTPUT_DIR ${CMAKE_CURRENT_BINARY_DIR}/dist)
file(MAKE_DIRECTORY ${PACKAGE_OUTPUT_DIR})

if (NOT PACKAGING_DIR)
    set(PACKAGING_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../packaging/windows/)
endif ()

configure_file(${PACKAGING_DIR}/installer-script.nsis.in
        ${PACKAGING_DIR}/installer-script.nsis
        @ONLY)

add_custom_target(dist_windows_install_before_packaging ALL
        DEPENDS SubconsciousShield
        COMMAND ${CMAKE_COMMAND} --install . --prefix ${PACKAGING_DIR}
        WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
        COMMENT "Installing the project into ${PACKAGING_DIR} before runing makensis.")

add_custom_target(dist ALL
        DEPENDS dist_windows_install_before_packaging
        COMMAND ${MAKENSIS_COMMAND} -V4
        -DVERSION=${PROJECT_VERSION}
        -DOUT=${PACKAGE_OUTPUT_DIR}/${PACKAGE_NAME}.exe
        installer-script.nsis
        WORKING_DIRECTORY ${PACKAGING_DIR}
        COMMENT "Generating NSIS package in ${PACKAGING_DIR}")

#
# Install
#
install(TARGETS SubconsciousShield DESTINATION Shield)
install(FILES ${INF_FILE} ${CATALOG_FILE} DESTINATION Shield)
