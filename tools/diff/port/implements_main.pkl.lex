Interface 0:9
Ident 10:15 text="Shape"
LBrace 16:17
Newline 17:18
Fn 22:24
Ident 25:29 text="area"
LParen 29:30
RParen 30:31
Arrow 32:34
Ident 35:40 text="float"
Newline 40:41
Fn 45:47
Ident 48:53 text="lines"
LParen 53:54
RParen 54:55
Arrow 56:58
Ident 59:62 text="int"
Newline 62:63
RBrace 63:64
Newline 64:65
Newline 65:66
Interface 66:75
Ident 76:81 text="Named"
LBrace 82:83
Newline 83:84
Fn 88:90
Ident 91:96 text="label"
LParen 96:97
RParen 97:98
Arrow 99:101
Ident 102:108 text="string"
Newline 108:109
RBrace 109:110
Newline 110:111
Newline 111:112
Class 112:117
Ident 118:124 text="Circle"
Implements 125:135
Ident 136:141 text="Shape"
LBrace 142:143
Newline 143:144
Var 148:151
Ident 152:158 text="radius"
Colon 158:159
Ident 160:165 text="float"
Newline 165:166
Newline 166:167
Fn 171:173
Ident 174:178 text="area"
LParen 178:179
RParen 179:180
Arrow 181:183
Ident 184:189 text="float"
LBrace 190:191
Newline 191:192
Number 200:210 text="3.14159265" suf="" isf=1 int=-
Star 211:212
This 213:217
Dot 217:218
Ident 218:224 text="radius"
Star 225:226
This 227:231
Dot 231:232
Ident 232:238 text="radius"
Newline 238:239
RBrace 243:244
Newline 244:245
Newline 245:246
Fn 250:252
Ident 253:258 text="lines"
LParen 258:259
RParen 259:260
Arrow 261:263
Ident 264:267 text="int"
LBrace 268:269
Newline 269:270
Number 278:279 text="0" suf="" isf=0 int=0
Newline 279:280
RBrace 284:285
Newline 285:286
RBrace 286:287
Newline 287:288
Newline 288:289
Class 289:294
Ident 295:299 text="Rect"
Implements 300:310
Ident 311:316 text="Shape"
Comma 316:317
Ident 318:323 text="Named"
LBrace 324:325
Newline 325:326
Var 330:333
Ident 334:335 text="w"
Colon 335:336
Ident 337:342 text="float"
Newline 342:343
Var 347:350
Ident 351:352 text="h"
Colon 352:353
Ident 354:359 text="float"
Newline 359:360
Newline 360:361
Fn 365:367
Ident 368:372 text="area"
LParen 372:373
RParen 373:374
Arrow 375:377
Ident 378:383 text="float"
LBrace 384:385
Newline 385:386
This 394:398
Dot 398:399
Ident 399:400 text="w"
Star 401:402
This 403:407
Dot 407:408
Ident 408:409 text="h"
Newline 409:410
RBrace 414:415
Newline 415:416
Newline 416:417
Fn 421:423
Ident 424:429 text="lines"
LParen 429:430
RParen 430:431
Arrow 432:434
Ident 435:438 text="int"
LBrace 439:440
Newline 440:441
Number 449:450 text="4" suf="" isf=0 int=4
Newline 450:451
RBrace 455:456
Newline 456:457
Newline 457:458
Fn 462:464
Ident 465:470 text="label"
LParen 470:471
RParen 471:472
Arrow 473:475
Ident 476:482 text="string"
LBrace 483:484
Newline 484:485
Str 493:499 T("rect")
Newline 499:500
RBrace 504:505
Newline 505:506
RBrace 506:507
Newline 507:508
Newline 508:509
Class 509:514
Ident 515:521 text="Square"
Extends 522:529
Ident 530:534 text="Rect"
LBrace 535:536
Newline 536:537
Override 541:549
Fn 550:552
Ident 553:558 text="label"
LParen 558:559
RParen 559:560
Arrow 561:563
Ident 564:570 text="string"
LBrace 571:572
Newline 572:573
Str 581:589 T("square")
Newline 589:590
RBrace 594:595
Newline 595:596
RBrace 596:597
Newline 597:598
Newline 598:599
Fn 599:601
Ident 602:608 text="areaOf"
LParen 608:609
Ident 609:610 text="s"
Colon 610:611
Ident 612:617 text="Shape"
RParen 617:618
Arrow 619:621
Ident 622:627 text="float"
LBrace 628:629
Newline 629:630
Ident 634:635 text="s"
Dot 635:636
Ident 636:640 text="area"
LParen 640:641
RParen 641:642
Newline 642:643
RBrace 643:644
Newline 644:645
Newline 645:646
Fn 646:648
Ident 649:656 text="linesOf"
LParen 656:657
Ident 657:658 text="s"
Colon 658:659
Ident 660:665 text="Shape"
RParen 665:666
Arrow 667:669
Ident 670:673 text="int"
LBrace 674:675
Newline 675:676
Ident 680:681 text="s"
Dot 681:682
Ident 682:687 text="lines"
LParen 687:688
RParen 688:689
Newline 689:690
RBrace 690:691
Newline 691:692
Newline 692:693
Fn 693:695
Ident 696:702 text="nameOf"
LParen 702:703
Ident 703:704 text="n"
Colon 704:705
Ident 706:711 text="Named"
RParen 711:712
Arrow 713:715
Ident 716:722 text="string"
LBrace 723:724
Newline 724:725
Ident 729:730 text="n"
Dot 730:731
Ident 731:736 text="label"
LParen 736:737
RParen 737:738
Newline 738:739
RBrace 739:740
Newline 740:741
Newline 741:742
Fn 742:744
Ident 745:749 text="main"
LParen 749:750
RParen 750:751
LBrace 752:753
Newline 753:754
Ident 758:765 text="println"
LParen 765:766
Ident 766:772 text="areaOf"
LParen 772:773
Ident 773:779 text="Circle"
LParen 779:780
Number 780:783 text="2.0" suf="" isf=1 int=-
RParen 783:784
RParen 784:785
Comma 785:786
Ident 787:793 text="areaOf"
LParen 793:794
Ident 794:798 text="Rect"
LParen 798:799
Number 799:802 text="3.0" suf="" isf=1 int=-
Comma 802:803
Number 804:807 text="4.0" suf="" isf=1 int=-
RParen 807:808
RParen 808:809
Comma 809:810
Ident 811:817 text="areaOf"
LParen 817:818
Ident 818:824 text="Square"
LParen 824:825
Number 825:828 text="2.0" suf="" isf=1 int=-
Comma 828:829
Number 830:833 text="5.0" suf="" isf=1 int=-
RParen 833:834
RParen 834:835
RParen 835:836
Newline 836:837
Ident 841:848 text="println"
LParen 848:849
Ident 849:856 text="linesOf"
LParen 856:857
Ident 857:863 text="Circle"
LParen 863:864
Number 864:867 text="1.0" suf="" isf=1 int=-
RParen 867:868
RParen 868:869
Comma 869:870
Ident 871:878 text="linesOf"
LParen 878:879
Ident 879:883 text="Rect"
LParen 883:884
Number 884:887 text="3.0" suf="" isf=1 int=-
Comma 887:888
Number 889:892 text="4.0" suf="" isf=1 int=-
RParen 892:893
RParen 893:894
Comma 894:895
Ident 896:903 text="linesOf"
LParen 903:904
Ident 904:910 text="Square"
LParen 910:911
Number 911:914 text="2.0" suf="" isf=1 int=-
Comma 914:915
Number 916:919 text="5.0" suf="" isf=1 int=-
RParen 919:920
RParen 920:921
RParen 921:922
Newline 922:923
Ident 927:934 text="println"
LParen 934:935
Ident 935:941 text="nameOf"
LParen 941:942
Ident 942:946 text="Rect"
LParen 946:947
Number 947:950 text="1.0" suf="" isf=1 int=-
Comma 950:951
Number 952:955 text="1.0" suf="" isf=1 int=-
RParen 955:956
RParen 956:957
Comma 957:958
Ident 959:965 text="nameOf"
LParen 965:966
Ident 966:972 text="Square"
LParen 972:973
Number 973:976 text="1.0" suf="" isf=1 int=-
Comma 976:977
Number 978:981 text="1.0" suf="" isf=1 int=-
RParen 981:982
RParen 982:983
RParen 983:984
Newline 984:985
Ident 989:996 text="println"
LParen 996:997
Ident 997:1003 text="Circle"
LParen 1003:1004
Number 1004:1007 text="1.0" suf="" isf=1 int=-
RParen 1007:1008
Is 1009:1011
Ident 1012:1017 text="Shape"
Comma 1017:1018
Ident 1019:1023 text="Rect"
LParen 1023:1024
Number 1024:1027 text="1.0" suf="" isf=1 int=-
Comma 1027:1028
Number 1029:1032 text="1.0" suf="" isf=1 int=-
RParen 1032:1033
Is 1034:1036
Ident 1037:1042 text="Shape"
Comma 1042:1043
Ident 1044:1050 text="Square"
LParen 1050:1051
Number 1051:1054 text="1.0" suf="" isf=1 int=-
Comma 1054:1055
Number 1056:1059 text="1.0" suf="" isf=1 int=-
RParen 1059:1060
Is 1061:1063
Ident 1064:1069 text="Shape"
RParen 1069:1070
Newline 1070:1071
Ident 1075:1082 text="println"
LParen 1082:1083
Ident 1083:1087 text="Rect"
LParen 1087:1088
Number 1088:1091 text="1.0" suf="" isf=1 int=-
Comma 1091:1092
Number 1093:1096 text="1.0" suf="" isf=1 int=-
RParen 1096:1097
Is 1098:1100
Ident 1101:1106 text="Named"
Comma 1106:1107
Ident 1108:1114 text="Square"
LParen 1114:1115
Number 1115:1118 text="1.0" suf="" isf=1 int=-
Comma 1118:1119
Number 1120:1123 text="1.0" suf="" isf=1 int=-
RParen 1123:1124
Is 1125:1127
Ident 1128:1133 text="Named"
RParen 1133:1134
Newline 1134:1135
Newline 1135:1136
Var 1140:1143
Ident 1144:1145 text="s"
Colon 1145:1146
Ident 1147:1152 text="Shape"
Assign 1153:1154
Ident 1155:1159 text="Rect"
LParen 1159:1160
Number 1160:1163 text="2.0" suf="" isf=1 int=-
Comma 1163:1164
Number 1165:1168 text="3.0" suf="" isf=1 int=-
RParen 1168:1169
As 1170:1172
Ident 1173:1178 text="Shape"
Newline 1178:1179
Var 1183:1186
Ident 1187:1188 text="r"
Colon 1188:1189
Ident 1190:1194 text="Rect"
Assign 1195:1196
Ident 1197:1198 text="s"
As 1199:1201
Ident 1202:1206 text="Rect"
Newline 1206:1207
Ident 1211:1218 text="println"
LParen 1218:1219
Ident 1219:1220 text="r"
Dot 1220:1221
Ident 1221:1222 text="w"
Comma 1222:1223
Ident 1224:1225 text="r"
Dot 1225:1226
Ident 1226:1227 text="h"
RParen 1227:1228
Newline 1228:1229
Ident 1233:1240 text="println"
LParen 1240:1241
Ident 1241:1242 text="s"
Is 1243:1245
Ident 1246:1250 text="Rect"
Comma 1250:1251
Ident 1252:1253 text="s"
Is 1254:1256
Ident 1257:1263 text="Circle"
RParen 1263:1264
Newline 1264:1265
RBrace 1265:1266
Eof 1266:1266
