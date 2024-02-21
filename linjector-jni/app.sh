#!/bin/bash

# set -x
# set -e

APP_NAME="com.github.erfur.lasso"

# Set the working directory to the location of this script
# cd "$(dirname "$0")"

# switch on the argument given
case "$1" in
    start)
        # Start the application
        # echo "Starting application"
        # start application with adb
        # adb shell am start -n $APP_NAME/$APP_NAME.SplashActivity
        # start application with monkey
        # suppress output
        adb shell monkey -p $APP_NAME -c android.intent.category.LAUNCHER 1 > /dev/null 2>&1
        sleep 1
        # fetch the pid of the application process
        PID=$(adb shell ps | grep $APP_NAME | grep root | awk '{print $2}')
        echo $PID
        ;;
    stop)
        # Stop the application
        echo "Stopping application"
        adb shell am force-stop $APP_NAME
        ;;
    *)
        # Print usage
        echo "Usage: $0 <command>"
        echo " Commands:"
        echo "  start - Start the application"
        echo "  stop - Kill the application"
        exit 1
        ;;
esac
