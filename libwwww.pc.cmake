prefix=@CMAKE_INSTALL_PREFIX@
exec_prefix=${prefix}/bin
libdir=${prefix}/@LIB_DESTINATION@
includedir=${prefix}/include/wwww
lib=wwww

Name: libwwww
Description: wwww - a C++ search engine, ported from the popular Apache Lucene
Version: @wwww_VERSION@
Libs: -L${prefix}/@LIB_DESTINATION@ -l${lib}
Cflags: -I${includedir}

