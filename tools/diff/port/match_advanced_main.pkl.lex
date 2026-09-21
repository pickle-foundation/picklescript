Enum 0:4
Ident 5:9 text="Dir2"
LBrace 10:11
Newline 11:12
Ident 16:21 text="North"
Newline 21:22
Ident 26:31 text="South"
Newline 31:32
Ident 36:40 text="East"
Newline 40:41
Ident 45:49 text="West"
Newline 49:50
RBrace 50:51
Newline 51:52
Newline 52:53
Fn 53:55
Ident 56:63 text="sumPair"
LParen 63:64
Ident 64:65 text="p"
Colon 65:66
LParen 67:68
Ident 68:71 text="int"
Comma 71:72
Ident 73:76 text="int"
RParen 76:77
RParen 77:78
Arrow 79:81
Ident 82:85 text="int"
LBrace 86:87
Newline 87:88
Match 92:97
LParen 98:99
Ident 99:100 text="p"
RParen 100:101
LBrace 102:103
Newline 103:104
Case 112:116
LParen 117:118
Ident 118:119 text="a"
Comma 119:120
Ident 121:122 text="b"
RParen 122:123
Arrow 124:126
Ident 127:128 text="a"
Plus 129:130
Ident 131:132 text="b"
Newline 132:133
RBrace 137:138
Newline 138:139
RBrace 139:140
Newline 140:141
Newline 141:142
Fn 142:144
Ident 145:152 text="labelOf"
LParen 152:153
Ident 153:154 text="p"
Colon 154:155
LParen 156:157
Ident 157:160 text="int"
Comma 160:161
Ident 162:165 text="int"
RParen 165:166
RParen 166:167
Arrow 168:170
Ident 171:177 text="string"
LBrace 178:179
Newline 179:180
Match 184:189
LParen 190:191
Ident 191:192 text="p"
RParen 192:193
LBrace 194:195
Newline 195:196
Case 204:208
LParen 209:210
Number 210:211 text="0" suf="" isf=0 int=0
Comma 211:212
Number 213:214 text="0" suf="" isf=0 int=0
RParen 214:215
Arrow 216:218
Str 219:227 T("origin")
Newline 227:228
Case 236:240
LParen 241:242
Ident 242:243 text="a"
Comma 243:244
Ident 245:246 text="_"
RParen 246:247
If 248:250
Ident 251:252 text="a"
Gt 253:254
Number 255:256 text="0" suf="" isf=0 int=0
Arrow 257:259
Str 260:267 T("right")
Newline 267:268
Case 276:280
Ident 281:282 text="_"
Arrow 283:285
Str 286:293 T("other")
Newline 293:294
RBrace 298:299
Newline 299:300
RBrace 300:301
Newline 301:302
Newline 302:303
Fn 303:305
Ident 306:313 text="dirName"
LParen 313:314
Ident 314:315 text="d"
Colon 315:316
Ident 317:321 text="Dir2"
RParen 321:322
Arrow 323:325
Ident 326:332 text="string"
LBrace 333:334
Newline 334:335
Match 339:344
LParen 345:346
Ident 346:347 text="d"
RParen 347:348
LBrace 349:350
Newline 350:351
Case 359:363
Ident 364:368 text="Dir2"
Dot 368:369
Ident 369:374 text="North"
Pipe 375:376
Ident 377:381 text="Dir2"
Dot 381:382
Ident 382:387 text="South"
Arrow 388:390
Str 391:401 T("vertical")
Newline 401:402
Case 410:414
Ident 415:419 text="Dir2"
Dot 419:420
Ident 420:424 text="East"
Pipe 425:426
Ident 427:431 text="Dir2"
Dot 431:432
Ident 432:436 text="West"
Arrow 437:439
Str 440:452 T("horizontal")
Newline 452:453
RBrace 457:458
Newline 458:459
RBrace 459:460
Newline 460:461
Newline 461:462
Fn 462:464
Ident 465:473 text="oneOrTwo"
LParen 473:474
Ident 474:475 text="n"
Colon 475:476
Ident 477:480 text="int"
RParen 480:481
Arrow 482:484
Ident 485:491 text="string"
LBrace 492:493
Newline 493:494
Match 498:503
LParen 504:505
Ident 505:506 text="n"
RParen 506:507
LBrace 508:509
Newline 509:510
Case 518:522
Number 523:524 text="1" suf="" isf=0 int=1
Pipe 525:526
Number 527:528 text="2" suf="" isf=0 int=2
Arrow 529:531
Str 532:539 T("small")
Newline 539:540
Case 548:552
Number 553:554 text="3" suf="" isf=0 int=3
Arrow 555:557
Str 558:565 T("three")
Newline 565:566
Case 574:578
Ident 579:580 text="_"
Arrow 581:583
Str 584:591 T("other")
Newline 591:592
RBrace 596:597
Newline 597:598
RBrace 598:599
Newline 599:600
Newline 600:601
Fn 601:603
Ident 604:614 text="noDupFirst"
LParen 614:615
Ident 615:616 text="m"
Colon 616:617
Ident 618:621 text="int"
Question 621:622
RParen 622:623
Arrow 624:626
Ident 627:633 text="string"
LBrace 634:635
Newline 635:636
Match 640:645
LParen 646:647
Ident 647:648 text="m"
RParen 648:649
LBrace 650:651
Newline 651:652
Case 660:664
Ident 665:669 text="some"
LParen 669:670
Number 670:671 text="1" suf="" isf=0 int=1
RParen 671:672
Pipe 673:674
Ident 675:679 text="some"
LParen 679:680
Number 680:681 text="2" suf="" isf=0 int=2
RParen 681:682
Arrow 683:685
Str 686:693 T("small")
Newline 693:694
Case 702:706
Ident 707:711 text="some"
LParen 711:712
Ident 712:713 text="v"
RParen 713:714
Arrow 715:717
Str 718:727 T("big:") E[Ident 724:725 text="v"; Eof 726:726]
Newline 727:728
Case 736:740
None 741:745
Arrow 746:748
Str 749:755 T("none")
Newline 755:756
RBrace 760:761
Newline 761:762
RBrace 762:763
Newline 763:764
Newline 764:765
Fn 765:767
Ident 768:779 text="sumOfOption"
LParen 779:780
Ident 780:781 text="p"
Colon 781:782
LParen 783:784
Ident 784:787 text="int"
Comma 787:788
Ident 789:795 text="string"
RParen 795:796
Question 796:797
RParen 797:798
Arrow 799:801
Ident 802:808 text="string"
LBrace 809:810
Newline 810:811
Match 815:820
LParen 821:822
Ident 822:823 text="p"
RParen 823:824
LBrace 825:826
Newline 826:827
Case 835:839
Ident 840:844 text="some"
LParen 844:845
LParen 845:846
Ident 846:847 text="a"
Comma 847:848
Ident 849:850 text="b"
RParen 850:851
RParen 851:852
Arrow 853:855
Str 856:865 E[Ident 858:859 text="a"; Eof 860:860] T(":") E[Ident 862:863 text="b"; Eof 864:864]
Newline 865:866
Case 874:878
None 879:883
Arrow 884:886
Str 887:896 T("missing")
Newline 896:897
RBrace 901:902
Newline 902:903
RBrace 903:904
Newline 904:905
Newline 905:906
Fn 906:908
Ident 909:913 text="main"
LParen 913:914
RParen 914:915
LBrace 916:917
Newline 917:918
Ident 922:929 text="println"
LParen 929:930
Ident 930:937 text="sumPair"
LParen 937:938
LParen 938:939
Number 939:940 text="3" suf="" isf=0 int=3
Comma 940:941
Number 942:943 text="4" suf="" isf=0 int=4
RParen 943:944
RParen 944:945
RParen 945:946
Newline 946:947
Ident 951:958 text="println"
LParen 958:959
Ident 959:966 text="labelOf"
LParen 966:967
LParen 967:968
Number 968:969 text="0" suf="" isf=0 int=0
Comma 969:970
Number 971:972 text="0" suf="" isf=0 int=0
RParen 972:973
RParen 973:974
Comma 974:975
Ident 976:983 text="labelOf"
LParen 983:984
LParen 984:985
Number 985:986 text="2" suf="" isf=0 int=2
Comma 986:987
Number 988:989 text="9" suf="" isf=0 int=9
RParen 989:990
RParen 990:991
Comma 991:992
Ident 993:1000 text="labelOf"
LParen 1000:1001
LParen 1001:1002
Minus 1002:1003
Number 1003:1004 text="1" suf="" isf=0 int=1
Comma 1004:1005
Number 1006:1007 text="1" suf="" isf=0 int=1
RParen 1007:1008
RParen 1008:1009
RParen 1009:1010
Newline 1010:1011
Ident 1015:1022 text="println"
LParen 1022:1023
Ident 1023:1030 text="dirName"
LParen 1030:1031
Ident 1031:1035 text="Dir2"
Dot 1035:1036
Ident 1036:1041 text="North"
RParen 1041:1042
Comma 1042:1043
Ident 1044:1051 text="dirName"
LParen 1051:1052
Ident 1052:1056 text="Dir2"
Dot 1056:1057
Ident 1057:1062 text="South"
RParen 1062:1063
Comma 1063:1064
Ident 1065:1072 text="dirName"
LParen 1072:1073
Ident 1073:1077 text="Dir2"
Dot 1077:1078
Ident 1078:1082 text="East"
RParen 1082:1083
Comma 1083:1084
Ident 1085:1092 text="dirName"
LParen 1092:1093
Ident 1093:1097 text="Dir2"
Dot 1097:1098
Ident 1098:1102 text="West"
RParen 1102:1103
RParen 1103:1104
Newline 1104:1105
Ident 1109:1116 text="println"
LParen 1116:1117
Ident 1117:1125 text="oneOrTwo"
LParen 1125:1126
Number 1126:1127 text="1" suf="" isf=0 int=1
RParen 1127:1128
Comma 1128:1129
Ident 1130:1138 text="oneOrTwo"
LParen 1138:1139
Number 1139:1140 text="2" suf="" isf=0 int=2
RParen 1140:1141
Comma 1141:1142
Ident 1143:1151 text="oneOrTwo"
LParen 1151:1152
Number 1152:1153 text="3" suf="" isf=0 int=3
RParen 1153:1154
Comma 1154:1155
Ident 1156:1164 text="oneOrTwo"
LParen 1164:1165
Number 1165:1166 text="9" suf="" isf=0 int=9
RParen 1166:1167
RParen 1167:1168
Newline 1168:1169
Ident 1173:1180 text="println"
LParen 1180:1181
Ident 1181:1191 text="noDupFirst"
LParen 1191:1192
Number 1192:1193 text="1" suf="" isf=0 int=1
As 1194:1196
Question 1196:1197
Ident 1198:1201 text="int"
RParen 1201:1202
Comma 1202:1203
Ident 1204:1214 text="noDupFirst"
LParen 1214:1215
Number 1215:1216 text="7" suf="" isf=0 int=7
As 1217:1219
Question 1219:1220
Ident 1221:1224 text="int"
RParen 1224:1225
Comma 1225:1226
Ident 1227:1237 text="noDupFirst"
LParen 1237:1238
None 1238:1242
RParen 1242:1243
RParen 1243:1244
Newline 1244:1245
Ident 1249:1256 text="println"
LParen 1256:1257
Ident 1257:1268 text="sumOfOption"
LParen 1268:1269
LParen 1269:1270
Number 1270:1271 text="7" suf="" isf=0 int=7
Comma 1271:1272
Str 1273:1276 T("x")
RParen 1276:1277
As 1278:1280
Question 1280:1281
LParen 1282:1283
Ident 1283:1286 text="int"
Comma 1286:1287
Ident 1288:1294 text="string"
RParen 1294:1295
RParen 1295:1296
Comma 1296:1297
Ident 1298:1309 text="sumOfOption"
LParen 1309:1310
None 1310:1314
RParen 1314:1315
RParen 1315:1316
Newline 1316:1317
RBrace 1317:1318
Eof 1318:1318
