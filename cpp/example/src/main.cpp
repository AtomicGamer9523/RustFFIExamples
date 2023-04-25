#include "logger.cpp"
#include <string>

using namespace std;

int main(int argc, const char** argv) {
    Logger logger = Logger::create(LOG_LEVEL_TRACE);
    logger.trace("Hello, world!");
    logger.debug("Hello, world!");
    logger.info("Hello, world!");
    logger.warn("Hello, world!");
    logger.error("Hello, world!");
    logger.fatal("Hello, world!");
}