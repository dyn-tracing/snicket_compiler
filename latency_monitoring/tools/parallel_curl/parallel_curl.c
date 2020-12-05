#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>
#include <string.h>
#include <time.h>
#include <stdint.h>
#include <inttypes.h>

#define BILLION  1000000000L
#define NUM_THREADS 50
 
/* create thread argument struct for thr_func() */
typedef struct _thread_data_t {
  int tid;
  double stuff;
  char* url;
} thread_data_t;
 
/* thread function */
void *thr_func(void *arg) {
  thread_data_t *data = (thread_data_t *)arg;
  // curl the url
  char cmd[100] = "curl -s ";
  strcat(cmd, data->url);
  strcat(cmd, " >> out > out2");
  int status = system(cmd);
  pthread_exit(NULL);
}

void send_burst(char* url) {
  pthread_t thr[NUM_THREADS];
  int i, rc;
  /* create a thread_data_t argument array */
  thread_data_t thr_data[NUM_THREADS];
 
  /* create threads */
  for (i = 0; i < NUM_THREADS; ++i) {
    thr_data[i].tid = i;
    thr_data[i].url = url;
    if ((rc = pthread_create(&thr[i], NULL, thr_func, &thr_data[i]))) {
      fprintf(stderr, "error: pthread_create, rc: %d\n", rc);
      exit(1);
    }
  }
  /* block until all threads complete */
  for (i = 0; i < NUM_THREADS; ++i) {
    pthread_join(thr[i], NULL);
  }

}
void print_time() {
  long int ns;
  uint64_t all;
  time_t sec;
  struct timespec spec;

  clock_gettime(CLOCK_REALTIME, &spec);
  sec = spec.tv_sec;
  ns = spec.tv_nsec;

  all = (uint64_t) sec * BILLION + (uint64_t) ns;

  printf("Current time: %" PRIu64  " nanoseconds since the Epoch\n", all);
 
}

void sequential_curls(char* url) {
  char cmd[100] = "curl -s ";
  strcat(cmd, url);
  strcat(cmd, " >> out > out2");
  printf("starting curls at ");
  print_time(); 
  for( int i =0; i<10; i++) {
    system(cmd);
  }
  printf("ended regular curls\n");

}
int main(int argc, char **argv) {
  if (argc != 2) {
    printf("Usage: ./pc URL\n");
    exit(0);
  }
  // set up curl command
  printf("url %s\n", argv[1]);
  printf("number of simultaneous curls is %d\n", NUM_THREADS);
  sequential_curls(argv[1]);
  printf("sending burst at ");
  print_time();
  send_burst(argv[1]);
 
  return EXIT_SUCCESS;
}
