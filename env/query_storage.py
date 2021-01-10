#!/usr/bin/env python3
import signal
import os
import argparse
import logging
import requests

import run_experiment
import util

log = logging.getLogger(__name__)


def query_storage():
    storage_content = requests.get("http://localhost:8090/list")
    return storage_content


def main(_):
    util.kill_tcp_proc(8090)
    storage_proc = run_experiment.launch_storage_mon()

    log.info("Storage content:\n%s", query_storage().text)
    # kill the storage proc after the query
    os.killpg(os.getpgid(storage_proc.pid), signal.SIGINT)


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("-l", "--log-file", dest="log_file",
                        default="storage.log",
                        help="Specifies name of the log file.")
    parser.add_argument("-ll", "--log-level", dest="log_level",
                        default="INFO",
                        choices=["CRITICAL", "ERROR", "WARNING",
                                 "INFO", "DEBUG", "NOTSET"],
                        help="The log level to choose.")
    parser.add_argument("-p", "--platform", dest="platform",
                        default="KB",
                        choices=["MK", "GCP"],
                        help="Which platform to run the scripts on."
                        "MK is minikube, GCP is Google Cloud Compute")

    # Parse options and process argv
    arguments = parser.parse_args()
    # configure logging
    logging.basicConfig(filename=arguments.log_file,
                        format="%(levelname)s:%(message)s",
                        level=getattr(logging, arguments.log_level),
                        filemode="w")
    stderr_log = logging.StreamHandler()
    stderr_log.setFormatter(logging.Formatter("%(levelname)s:%(message)s"))
    logging.getLogger().addHandler(stderr_log)
    main(arguments)
