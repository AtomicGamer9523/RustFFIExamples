#include "../rust/bindings.h"
#include <string>

#ifndef __cplusplus

typedef struct Logger {} Logger;

#else

#define LOG_LEVEL_TRACE 0
#define LOG_LEVEL_DEBUG 1
#define LOG_LEVEL_INFO 2
#define LOG_LEVEL_WARN 3
#define LOG_LEVEL_ERROR 4
#define LOG_LEVEL_FATAL 5

class Logger {
private:
    int level;
    void log(int level, const char* msg) {
        if(this->level > level) return;
        rslog(level, msg);
    }
public:
    static Logger create() {
        return Logger(LOG_LEVEL_INFO);
    }
    static Logger create(const int lvl) {
        return Logger(lvl);
    }
    Logger(const int lvl) {
        level = lvl;
    }
    void trace(const char* msg) {
        this->log(LOG_LEVEL_TRACE, msg);
    }
    void debug(const char* msg) {
        this->log(LOG_LEVEL_DEBUG, msg);
    }
    void info(const char* msg) {
        this->log(LOG_LEVEL_INFO, msg);
    }
    void warn(const char* msg) {
        this->log(LOG_LEVEL_WARN, msg);
    }
    void error(const char* msg) {
        this->log(LOG_LEVEL_ERROR, msg);
    }
    void fatal(const char* msg) {
        this->log(LOG_LEVEL_FATAL, msg);
    }
};
#endif
