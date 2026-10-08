function upload_to_blob_store_with_acl
{
    bucket_name="$1"
    file_to_upload="$2"
    blob_store_key="$3"
    acl="$4"

    date=$(date +"%a, %d %b %Y %T %z")
    content_type="application/octet-stream"
    storage_type="x-amz-storage-class:STANDARD"
    string="PUT\n\n${content_type}\n${date}\n${acl}\n${storage_type}\n/${bucket_name}/${blob_store_key}"
    signature=$(echo -en "${string}" | openssl sha1 -hmac "${DIGITALOCEAN_SPACES_SECRET_KEY}" -binary | base64)

    curl --fail -vv -s -X PUT -T "$file_to_upload" \
        -H "Host: ${bucket_name}.nyc3.digitaloceanspaces.com" \
        -H "Date: $date" \
        -H "Content-Type: $content_type" \
        -H "$storage_type" \
        -H "$acl" \
        -H "Authorization: AWS ${DIGITALOCEAN_SPACES_ACCESS_KEY}:$signature" \
        "https://${bucket_name}.nyc3.digitaloceanspaces.com/${blob_store_key}"
}

function upload_to_blob_store_public
{
    upload_to_blob_store_with_acl "$1" "$2" "$3" "x-amz-acl:public-read"
}

function upload_to_blob_store
{
    upload_to_blob_store_with_acl "$1" "$2" "$3" "x-amz-acl:private"
}

# aacode 备注：此文件原样取自 Zed `script/lib/blob-store.sh`，用于把构建产物上传到
# DigitalOcean Spaces。bundle-linux 只 `source` 它、并不实际调用上传函数
# （上传由 Zed 的发布流程单独触发）。aacode 若要发布，填 DIGITALOCEAN_SPACES_*
# 环境变量即可；不发布则这些函数不会被调用。
