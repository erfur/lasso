// Write C++ code here.
//
// Do not forget to dynamically load the C++ library into your application.
//
// For instance,
//
// In MainActivity.java:
//    static {
//       System.loadLibrary("lasso");
//    }
//
// Or, in MainActivity.kt:
//    companion object {
//      init {
//         System.loadLibrary("lasso")
//      }
//    }

#include <jni.h>
#include <string>
#include <android/log.h>

__attribute__((visibility("default")))
void *test_var = NULL;

extern "C"
JNIEXPORT void JNICALL
Java_com_github_erfur_lasso_MainActivity_testFunction(JNIEnv *env, jobject thiz) {
    // log
    __android_log_print(ANDROID_LOG_DEBUG, "Lasso", "testFunction");
    // create a new string byte by byte
    char *str = new char[10];
    for (int i = 0; i < 10; i++) {
        str[i] = 'a' + i;
    }
}