#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>
#include <string.h>

#define NUM_THREADS 20
 
/* create thread argument struct for thr_func() */
typedef struct _thread_data_t {
  int tid;
  double stuff;
  char* url;
} thread_data_t;
 
/* thread function */
void *thr_func(void *arg) {
  thread_data_t *data = (thread_data_t *)arg;
  printf("hello from thr_func, thread id: %d, url: %s \n", data->tid, data->url);
  // curl the url
  char cmd[100] = "curl -v ";
  strcat(cmd, data->url);
  printf("command is %s\n", cmd);
  int status = system(cmd);
  pthread_exit(NULL);
}
 
int main(int argc, char **argv) {
  if (argc != 2) {
    printf("Usage: ./pc URL\n");
    exit(0);
  }
  printf("url %s\n", argv[1]);

  pthread_t thr[NUM_THREADS];
  int i, rc;
  /* create a thread_data_t argument array */
  thread_data_t thr_data[NUM_THREADS];
 
  /* create threads */
  for (i = 0; i < NUM_THREADS; ++i) {
    thr_data[i].tid = i;
    thr_data[i].url = argv[1];
    if ((rc = pthread_create(&thr[i], NULL, thr_func, &thr_data[i]))) {
      fprintf(stderr, "error: pthread_create, rc: %d\n", rc);
      return EXIT_FAILURE;
    }
  }
  /* block until all threads complete */
  for (i = 0; i < NUM_THREADS; ++i) {
    pthread_join(thr[i], NULL);
  }
 
  return EXIT_SUCCESS;
}
