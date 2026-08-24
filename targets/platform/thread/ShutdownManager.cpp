#include "ShutdownManager.h"

extern "C" {
void rust_shutdown_manager_request_shutdown();
void rust_shutdown_manager_request_restart();
bool rust_shutdown_manager_is_shutdown_requested();
bool rust_shutdown_manager_is_restart_requested();
}

ShutdownManager::State& ShutdownManager::GetState() {
    static State state;
    return state;
}

void ShutdownManager::Initialise() {}

void ShutdownManager::StartShutdown() {
    rust_shutdown_manager_request_shutdown();
}

void ShutdownManager::MainThreadHandleShutdown() {}

void ShutdownManager::HasStarted(EThreadId threadId) {}

void ShutdownManager::HasStarted(EThreadId threadId,
                                  C4JThread::EventArray* eventArray) {}

bool ShutdownManager::ShouldRun(EThreadId threadId) {
    return !rust_shutdown_manager_is_shutdown_requested();
}

void ShutdownManager::HasFinished(EThreadId threadId) {}
