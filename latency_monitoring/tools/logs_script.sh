#!/usr/bin/env bash

gcloud logging read "resource.type=k8s_container AND labels.k8s-pod/app=reviews AND textPayload:Stored AND labels.k8s-pod/version=v3" --limit 200

gcloud logging read "resource.type=k8s_container AND labels.k8s-pod/app=reviews AND textPayload:Stored AND labels.k8s-pod/version=v2" --limit 200
