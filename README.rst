tsub
====
tsub reads text line-by-line from stdin and substitutes characters.
The characters to be swapped out are stored in a config file like so:

    My Substitution:AjEfEl
    Rot13:AZBOCQDSEUFWGYHAICJEKGLIMKRTVXZBDFHJLMNOPQRSTUVWXYNPazbocqdseufwgyhaicjekglimkrtvxzbdfhjlmnopqrstuvwxynp
    Lowercase:AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz
    Math bold italic:A𝑨B𝑩C𝑪D𝑫E𝑬F𝑭G𝑮H𝑯I𝑰J𝑱K𝑲L𝑳M𝑴N𝑵O𝑶P𝑷Q𝑸R𝑹S𝑺T𝑻U𝑼V𝑽W𝑾X𝑿Y𝒀Z𝒁a𝒂b𝒃c𝒄d𝒅e𝒆f𝒇g𝒈h𝒉i𝒊j𝒋k𝒌l𝒍m𝒎n𝒏o𝒐p𝒑q𝒒r𝒓s𝒔t𝒕u𝒖v𝒗w𝒘x𝒙y𝒚x𝒛00112233445566778899

Usage: ``tsub -c <config-file> -s <substitution>``

Use ``tsub -c <config-file> -l`` to list the names of the available substitutions in a config file.

A script has been included which will use dmenu (or bemenu, if installed) to allow a substitution to be selected with a menu and text enteted.
This relies on xsel to acquire the current clipboard text and to repopulate the clipboard with the newly created text.
