#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>


int main() {
    ssize_t read;
    char * line = NULL;
    size_t len = 0;

    int ok = 0;
    int nok = 0;
    int count = 0;

    printf("Starting read!\n");
    const char* source = "/Users/phiro/Dropbox/Projects/kubernetes/CKAD/training/6/scratch/testresults.jtl";
    FILE *file_handle = fopen(source, "r");
    while ((read = getline(&line, &len, file_handle)) != -1) {
        ++count;
        char* timestamp = strtok(line, ",");
        char* elapsed = strtok(NULL, ",");
        char* label = strtok(NULL, ",");
        char* resCode = strtok(NULL, ",");

        long timestamp_int = atol(timestamp)/1000;
        time_t time = timestamp_int;
        struct tm ts = *localtime(&time);
        char buf[80];
        strftime(buf, sizeof(buf), "%a %Y-%m-%d %H:%M:%S ep:%s %Z", &ts);

        if (strcmp(resCode, "200") == 0) {
            ++ok;
        } else {
            printf("Non 200 error: %s - timestamp: %s \n", resCode,buf);
            ++nok;
        }
        if (count % 1000000 == 0) {

            printf("Value epoch - time: %s (%s) resCode: %s, remainder %s:: ok:%d, nok:%d, total: %d\n", timestamp, buf, resCode, line, ok, nok, count);
        }
    }
    printf("Ending read, ok: %d, nok: %d, total records: %d!\n", ok, nok, count);
    fclose(file_handle);
    return 0;
}
