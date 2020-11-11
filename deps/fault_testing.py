#!/usr/bin/env python3
import argparse
import logging
import sys
from pathlib import Path

import util

log = logging.getLogger(__name__)

FILE_DIR = Path(__file__).parent.resolve()
ROOT_DIR = FILE_DIR.parent
ISTIO_DIR = FILE_DIR.joinpath("istio-1.7.4")
ISTIO_BIN = ISTIO_DIR.joinpath("bin/istioctl")


def main(args):
    # start minikube
    cmd = "minikube start"
    result = util.exec_process(cmd)

    # inject istio
    cmd = f"{ISTIO_BIN} install --set profile=demo"
    result = util.exec_process(cmd)
    cmd = f"kubectl label namespace default istio-injection=enabled"
    result = util.exec_process(cmd)

    # launch bookinfo
    cmd = f"kubectl apply -f {ISTIO_DIR}/samples/bookinfo/platform/kube/bookinfo.yaml && "
    cmd += f"kubectl apply -f {ISTIO_DIR}/samples/bookinfo/networking/bookinfo-gateway.yaml && "
    cmd += f"kubectl apply -f {ISTIO_DIR}/samples/bookinfo/networking/destination-rule-all.yaml && "
    cmd += f"kubectl apply -f {ISTIO_DIR}/samples/bookinfo/networking/destination-rule-all-mtls.yaml && "
    cmd += f"kubectl apply -f {ISTIO_DIR}/samples/httpbin/sample-client/fortio-deploy.yaml "
    result = util.exec_process(cmd)

    cmd = "/bin/bash -c 'echo $(kubectl get deploy -o name) | { read -d EOF deploy; for i in $deploy; do kubectl rollout status $i -w --timeout=180s; done; }'"
    result = util.exec_process(cmd)

    # delete minikube
    cmd = "minikube delete"
    result = util.exec_process(cmd)
    sys.exit(result)

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("-l", "--log_file", dest="log_file",
                        default="model.log",
                        help="Specifies name of the log file.")
    parser.add_argument("-ll", "--log_level", dest="log_level",
                        default="INFO",
                        choices=["CRITICAL", "ERROR", "WARNING",
                                 "INFO", "DEBUG", "NOTSET"],
                        help="The log level to choose.")
    # Parse options and process argv
    arguments = parser.parse_args()
    # configure logging
    logging.basicConfig(filename=arguments.log_file,
                        format="%(levelname)s:%(message)s",
                        level=getattr(logging, arguments.log_level),
                        filemode='w')
    stderr_log = logging.StreamHandler()
    stderr_log.setFormatter(logging.Formatter("%(levelname)s:%(message)s"))
    logging.getLogger().addHandler(stderr_log)
    main(arguments)
