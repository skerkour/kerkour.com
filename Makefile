.PHONY: exif
exif:
	bash -c 'exiftool -overwrite_original -recurse -all= public/* courses/* blog/* books/*/assets/*.{jpg,png} books/summaries/*.{jpg,png} || exit 0'
