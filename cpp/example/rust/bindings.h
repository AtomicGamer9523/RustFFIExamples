
#pragma once
#ifdef __cplusplus
extern "C" {
#endif
    /**
     * @brief logs a message to the console using rust's println! macro
     * Log levels are:
     * 0 - trace
     * 1 - debug
     * 2 - info
     * 3 - warn
     * 4 - error
     * 5 - fatal
     * @param level level at which to log the message (0-5), see above
     * @param msg message to log to the console
     */
    void rslog(int level, const char *msg);
#ifdef __cplusplus
}
#endif
