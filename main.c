#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define SHELL "lsh"
#define VERSION "lsh (Lightweight Shell) 0.1"

#define YEL "\e[0;33m"
#define RESET "\e[0m"

char *replaceWord(const char *s, const char *oldW, const char *newW) {
  char *result;
  int i, cnt = 0;
  int newWlen = strlen(newW);
  int oldWlen = strlen(oldW);

  for (i = 0; s[i] != '\0'; i++) {
    if (strstr(&s[i], oldW) == &s[i]) {
      cnt++;

      i += oldWlen - 1;
    }
  }

  result = (char *)malloc(i + cnt * (newWlen - oldWlen) + 1);
  i = 0;
  while (*s) {
    if (strstr(s, oldW) == s) {
      strcpy(&result[i], newW);
      i += newWlen;
      s += oldWlen;
    } else
      result[i++] = *s++;
  }

  result[i] = '\0';
  return result;
}

void handle_singal(int sig) {
  char dir[256];
  char *result = NULL;
  char s[100]; 
  char *user = getenv("USER");
  if (user != NULL) {
    strcpy(dir, "/home/");
    strcat(dir, user);
  }

  result = replaceWord(getenv("PWD"), dir, "~");
  printf(YEL "\n%s" RESET "@" YEL "%s $ " RESET, getenv("USER"), result);
  printf("Use \"exit\" command\n");
}

int main() {
  char var[50];
  char *result = NULL;
  char dir[256];
  char *user = getenv("USER");
  char s[100]; 

  char tags;

  if (signal(SIGINT, handle_singal) == SIG_ERR) {
    perror("Error");
    return 1;
  }

  for (;;) {
    if (user != NULL) {
      strcpy(dir, "/home/");
      strcat(dir, user);
    }

    result = replaceWord(getcwd(s, 100), dir, "~");
    printf(YEL "%s" RESET "@" YEL "%s $ " RESET, getenv("USER"), result);

    if (fgets(var, sizeof(var), stdin) == NULL) {
      break;
    }

    size_t len = strlen(var);
    if (len > 0 && var[len - 1] == '\n') {
      var[len - 1] = '\0';
    }

    if (strcmp(var, "lsh help") == 0) {
      printf(
          "help\tprint the help menu\nverison\toutput version information\n");

    } else if (strcmp(var, "help") == 0) {
      printf("Do you mean lsh help?\n");
    } else if (strcmp(var, "lsh version") == 0) {
      printf("%s\n", VERSION);
    } else if (strcmp(var, "exit") == 0) {
      return 0;
    } else if (strcmp(var, "ls") == 0) {
      system("ls --color");

    } else if (strcmp(var, "cd") == 0) {


      chdir("..");
    } else {
      strcat(var, " 2>/dev/null");
      int cmd = system(var);
      if (WIFEXITED(cmd)) {
        int exit_status = WEXITSTATUS(cmd);
        if (exit_status == 0) {
          printf("+\n");
        } else {
          result = replaceWord(var, "2>/dev/null", " ");
          printf("%s: command not found: %s\n", SHELL, result);
        }
      } else {
        printf("EXIT\n");
      }
    }
  }
  return 0;
}