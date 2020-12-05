#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <unistd.h>
#include <string.h>
#include <time.h>

#define NUM_THREADS 200
 
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
  strcat(cmd, " >> out > out2");
  printf("command is %s\n", cmd);
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
    int hours, minutes, seconds, day, month, year;
 
    time_t now;
 
    // Obtain current time
    // time() returns the current time of the system as a time_t value
    time(&now);
 
    // Convert to local time format and print to stdout
    printf("Today is : %s", ctime(&now));
 
    // localtime converts a time_t value to calendar time and
    // returns a pointer to a tm structure with its members
    // filled with the corresponding values
    struct tm *local = localtime(&now);
 
    hours = local->tm_hour;          // get hours since midnight (0-23)
    minutes = local->tm_min;         // get minutes passed after the hour (0-59)
    seconds = local->tm_sec;         // get seconds passed after minute (0-59)
 
    day = local->tm_mday;            // get day of month (1 to 31)
    month = local->tm_mon + 1;       // get month of year (0 to 11)
    year = local->tm_year + 1900;    // get year since 1900
 
    // print local time
    if (hours < 12)    // before midday
        printf("Time is : %02d:%02d:%02d am\n", hours, minutes, seconds);
 
    else    // after midday
        printf("Time is : %02d:%02d:%02d pm\n", hours - 12, minutes, seconds);

}

void sequential_curls(char* url) {
  char cmd[100] = "curl -v ";
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
  sequential_curls(argv[1]);
  printf("sending burst at ");
  print_time();
  send_burst(argv[1]);
 
  return EXIT_SUCCESS;
}
