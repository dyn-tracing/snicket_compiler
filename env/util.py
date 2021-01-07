import os
import subprocess
import shutil
import logging as log
from pathlib import Path
from datetime import datetime
import time


EXIT_SUCCESS = 0
EXIT_FAILURE = 1

TO_NANOSECONDS = 1e9  # everything is in nanoseconds


def is_valid_file(parser, arg):
    if not os.path.exists(arg):
        return parser.error("File %s does not exist!" % arg)
    else:
        return Path(arg)


def check_dir(directory):
    # create the folder if it does not exit
    if not directory == "" and not os.path.exists(directory):
        log.debug("Folder %s does not exist! Creating...", directory)
        directory.mkdir(parents=True, exist_ok=True)


def del_dir(directory):
    try:
        shutil.rmtree(directory, ignore_errors=True)
    except OSError as e:
        log.warning("Could not delete directory, reason:")
        log.warning("%s - %s.", e.filename, e.strerror)


def copy_file(src, dst):
    try:
        if isinstance(src, list):
            for src_file in src:
                shutil.copy2(src_file, dst)
        else:
            shutil.copy2(src, dst)
    except shutil.SameFileError:
        # this is fine
        pass


def move_file(src, dst):
    src = str(src)
    dst = str(dst)
    if isinstance(src, list):
        for src_file in src:
            src.move(src_file, dst)
    else:
        shutil.move(src, dst)


def get_output_from_proc(cmd, *args, **kwargs):
    log.debug("Executing %s ", cmd)
    return subprocess.check_output(cmd.split(), *args, **kwargs)


def start_process(cmd, *args, out_file=subprocess.PIPE, **kwargs):
    log.debug("Executing %s ", cmd)
    if out_file is subprocess.STDOUT:
        proc = subprocess.Popen(cmd.split(), *args, **kwargs)
    elif out_file is subprocess.PIPE:
        proc = subprocess.Popen(
            cmd.split(), stdout=out_file, stderr=out_file, *args, **kwargs)
    else:
        err = out_file + ".err"
        out = out_file + ".out"
        with open(out, "w+") as f_out, open(err, "w+") as f_err:
            proc = subprocess.Popen(cmd.split(), stdout=f_out,
                                    stderr=f_err, *args, **kwargs)
    return proc


def exec_process(cmd, *args, **kwargs):
    log.debug("Executing %s ", cmd)
    result = subprocess.run(cmd, shell=True, *args, **kwargs)
    if result.stdout:
        log.debug("Process output: %s", result.stdout.decode("utf-8"))
    if result.returncode != EXIT_SUCCESS:
        log.error("BEGIN %s", 40 * "#")
        log.error("Failed while executing:\n%s\n", cmd)
        if result.stderr:
            log.error("Output:\n%s", result.stderr.decode("utf-8"))
        log.error("END %s", 40 * "#")
    return result.returncode


def ns_to_timestamp(str_ns):
    ns = int(str_ns)
    dt = datetime.fromtimestamp(ns / 1e9)
    return f"{dt.strftime('%H:%M:%S.%f')}.{(ns % 1e3):03.0f}"


def nano_ts(offset=0):
    return ns_to_timestamp((time.time() * TO_NANOSECONDS) - offset)


def kill_tcp_proc(port_num):
    cmd = f"lsof -ti tcp:{port_num} | xargs kill || exit 0"
    return exec_process(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
