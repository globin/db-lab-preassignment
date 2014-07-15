VERSION = DEBUG

DEBUG_CFLAGS = -g -O0 -DUSE_HEAP_STATS -Wall
RELEASE_CFLAGS = -O3

ifeq "$(VERSION)" "DEBUG"
CFLAGS = $(DEBUG_CFLAGS)
else
ifeq "$(VERSION)" "RELEASE"
CFLAGS = $(RELEASE_CFLAGS)
endif
endif

CXX = g++
CC = $(CXX)
LD=g++
CXXFLAGS = -std=c++11 $(CFLAGS)
LDFLAGS=

APPNAME=hekaton

APPSOURCES=hekaton.cpp

SOURCES=$(APPSOURCES) $(SHAREDSOURCES)

SRCDIR = src/
OBJDIR = build/

OBJECTS=$(SOURCES:.cpp=.o)
	SOURCEFILES=$(addprefix $(SRCDIR),$(SOURCES))
	OBJECTFILES=$(addprefix $(OBJDIR),$(OBJECTS))


all: $(APPNAME)

clean:
	rm -rf $(OBJDIR)
	rm -rf $(APPNAME)

$(OBJDIR)%.o: $(SRCDIR)%.cpp
	$(CC) -c $(CXXFLAGS) -o $@ $^

$(OBJDIR):
	mkdir -p $@

$(APPNAME): $(OBJDIR) $(OBJECTFILES)
	$(LD) $(OBJECTFILES) $(LDFLAGS) -o $@
