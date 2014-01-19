#!/bin/sh

. `dirname $0`/mime-functions.sh

echo_mime () {
	printf "$i;";
}

printf MimeType=;

get_mimetypes $1;
for i in $MIMETYPES ; do
	echo_mime;
done

echo ""
