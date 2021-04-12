cmake_minimum_required(VERSION 3.0)
project(LibFlecs C)

add_library(flecs STATIC flecs.c)

install(TARGETS flecs DESTINATION .)
