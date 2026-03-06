#ifndef MAIN_H
#define MAIN_H

#include <stdio.h>
#include <stdlib.h>

/**
 * struct shell_data - structure to hold all shell related data
 * @input_line: pointer to the user input line
 * @num_byte: number of byte allowed for the input line
 * @args: array of command arguements
 * @env: pointer to array of strings of envirement variables
 * @argv: program arguements
 * @argc:number of program arguemets
 */
typedef struct shell_data
{
    char *input_line;
    size_t num_byte;
    char *args[10];
    char **env;
    char **argv;
    int argc;
} shell_data;