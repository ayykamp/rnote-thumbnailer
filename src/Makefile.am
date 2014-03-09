


AM_CPPFLAGS = -I$(top_srcdir)                                              \
           $(GNOME_CFLAGS) \
           $(OPENRAW_CFLAGS) \
           -DG_LOG_DOMAIN=\"raw-thumbnailer\"    \
           -DGNOMELOCALEDIR=\""$(prefix)/$(DATADIRNAME)/locale"\"       \
           -DDATADIR=\""$(datadir)"\"                                   \
           -DG_DISABLE_DEPRECATED -DGTK_DISABLE_DEPRECATED              \
           -DGDK_DISABLE_DEPRECATED -DGNOME_DISABLE_DEPRECATED


bin_PROGRAMS = raw-thumbnailer

raw_thumbnailer_SOURCES = raw-thumbnailer.c

raw_thumbnailer_LDADD = $(GNOME_LIBS) $(OPENRAW_LIBS) $(INTLLIBS)
