package com.github.erfur.lasso

class InjectorTestClass {

    private external fun testFunction()

    companion object {
        init {
            System.loadLibrary("lasso")
        }
    }

    init {
        // create a thread that runs testFunction periodically
        Thread {
            while (true) {
                Thread.sleep(1000)
                testFunction()
            }
        }.start()
    }
}