Class 0:5
Ident 6:13 text="Counter"
Implements 14:24
Ident 25:33 text="Iterable"
Lt 33:34
Ident 34:37 text="int"
Gt 37:38
Comma 38:39
Ident 40:48 text="Iterator"
Lt 48:49
Ident 49:52 text="int"
Gt 52:53
LBrace 54:55
Newline 55:56
Var 60:63
Ident 64:65 text="n"
Colon 65:66
Ident 67:70 text="int"
Newline 70:71
Newline 71:72
Fn 76:78
Ident 79:87 text="iterator"
LParen 87:88
RParen 88:89
Arrow 90:92
Ident 93:101 text="Iterator"
Lt 101:102
Ident 102:105 text="int"
Gt 105:106
LBrace 107:108
Newline 108:109
Return 117:123
This 124:128
Newline 128:129
RBrace 133:134
Newline 134:135
Newline 135:136
Fn 140:142
Ident 143:147 text="next"
LParen 147:148
RParen 148:149
Arrow 150:152
Ident 153:156 text="int"
Question 156:157
LBrace 158:159
Newline 159:160
If 168:170
LParen 171:172
Ident 172:173 text="n"
Le 174:176
Number 177:178 text="0" suf="" isf=0 int=0
RParen 178:179
LBrace 180:181
Newline 181:182
Return 194:200
None 201:205
Newline 205:206
RBrace 214:215
Newline 215:216
Let 224:227
Ident 228:229 text="v"
Assign 230:231
Ident 232:233 text="n"
Newline 233:234
Ident 242:243 text="n"
Assign 244:245
Ident 246:247 text="n"
Minus 248:249
Number 250:251 text="1" suf="" isf=0 int=1
Newline 251:252
Return 260:266
Ident 267:268 text="v"
Newline 268:269
RBrace 273:274
Newline 274:275
RBrace 275:276
Newline 276:277
Newline 277:278
Class 278:283
Ident 284:292 text="RangeSeq"
Implements 293:303
Ident 304:312 text="Iterable"
Lt 312:313
Ident 313:316 text="int"
Gt 316:317
LBrace 318:319
Newline 319:320
Var 324:327
Ident 328:330 text="lo"
Colon 330:331
Ident 332:335 text="int"
Newline 335:336
Var 340:343
Ident 344:346 text="hi"
Colon 346:347
Ident 348:351 text="int"
Newline 351:352
Newline 352:353
Fn 357:359
Ident 360:368 text="iterator"
LParen 368:369
RParen 369:370
Arrow 371:373
Ident 374:382 text="Iterator"
Lt 382:383
Ident 383:386 text="int"
Gt 386:387
LBrace 388:389
Newline 389:390
Return 398:404
Ident 405:414 text="RangeIter"
LParen 414:415
This 415:419
Comma 419:420
Ident 421:423 text="lo"
RParen 423:424
Newline 424:425
RBrace 429:430
Newline 430:431
RBrace 431:432
Newline 432:433
Newline 433:434
Class 434:439
Ident 440:449 text="RangeIter"
Implements 450:460
Ident 461:469 text="Iterator"
Lt 469:470
Ident 470:473 text="int"
Gt 473:474
LBrace 475:476
Newline 476:477
Var 481:484
Ident 485:488 text="seq"
Colon 488:489
Ident 490:498 text="RangeSeq"
Newline 498:499
Var 503:506
Ident 507:510 text="cur"
Colon 510:511
Ident 512:515 text="int"
Newline 515:516
Newline 516:517
Fn 521:523
Ident 524:528 text="next"
LParen 528:529
RParen 529:530
Arrow 531:533
Ident 534:537 text="int"
Question 537:538
LBrace 539:540
Newline 540:541
If 549:551
LParen 552:553
Ident 553:556 text="cur"
Ge 557:559
Ident 560:563 text="seq"
Dot 563:564
Ident 564:566 text="hi"
RParen 566:567
LBrace 568:569
Newline 569:570
Return 582:588
None 589:593
Newline 593:594
RBrace 602:603
Newline 603:604
Let 612:615
Ident 616:617 text="v"
Assign 618:619
Ident 620:623 text="cur"
Newline 623:624
Ident 632:635 text="cur"
Assign 636:637
Ident 638:641 text="cur"
Plus 642:643
Number 644:645 text="1" suf="" isf=0 int=1
Newline 645:646
Return 654:660
Ident 661:662 text="v"
Newline 662:663
RBrace 667:668
Newline 668:669
RBrace 669:670
Newline 670:671
Newline 671:672
Class 672:677
Ident 678:686 text="SubRange"
Extends 687:694
Ident 695:703 text="RangeSeq"
LBrace 704:705
RBrace 705:706
Newline 706:707
Newline 707:708
Fn 708:710
Ident 711:714 text="sum"
LParen 714:715
Ident 715:717 text="it"
Colon 717:718
Ident 719:727 text="Iterable"
Lt 727:728
Ident 728:731 text="int"
Gt 731:732
RParen 732:733
Arrow 734:736
Ident 737:740 text="int"
LBrace 741:742
Newline 742:743
Var 747:750
Ident 751:756 text="total"
Assign 757:758
Number 759:760 text="0" suf="" isf=0 int=0
Newline 760:761
For 765:768
LParen 769:770
Ident 770:771 text="x"
In 772:774
Ident 775:777 text="it"
RParen 777:778
LBrace 779:780
Newline 780:781
Ident 789:794 text="total"
Assign 795:796
Ident 797:802 text="total"
Plus 803:804
Ident 805:806 text="x"
Newline 806:807
RBrace 811:812
Newline 812:813
Ident 817:822 text="total"
Newline 822:823
RBrace 823:824
Newline 824:825
Newline 825:826
Fn 826:828
Ident 829:838 text="nextOrNeg"
LParen 838:839
Ident 839:841 text="it"
Colon 841:842
Ident 843:851 text="Iterator"
Lt 851:852
Ident 852:855 text="int"
Gt 855:856
RParen 856:857
Arrow 858:860
Ident 861:864 text="int"
LBrace 865:866
Newline 866:867
Let 871:874
Ident 875:876 text="v"
Assign 877:878
Ident 879:881 text="it"
Dot 881:882
Ident 882:886 text="next"
LParen 886:887
RParen 887:888
Newline 888:889
If 893:895
LParen 896:897
Let 897:900
Ident 901:905 text="some"
LParen 905:906
Ident 906:907 text="x"
RParen 907:908
Assign 909:910
Ident 911:912 text="v"
RParen 912:913
LBrace 914:915
Newline 915:916
Return 924:930
Ident 931:932 text="x"
Newline 932:933
RBrace 937:938
Newline 938:939
Return 943:949
Minus 950:951
Number 951:952 text="1" suf="" isf=0 int=1
Newline 952:953
RBrace 953:954
Newline 954:955
Newline 955:956
Fn 956:958
Ident 959:963 text="main"
LParen 963:964
RParen 964:965
LBrace 966:967
Newline 967:968
Ident 972:979 text="println"
LParen 979:980
Ident 980:983 text="sum"
LParen 983:984
Ident 984:991 text="Counter"
LParen 991:992
Number 992:993 text="3" suf="" isf=0 int=3
RParen 993:994
RParen 994:995
Comma 995:996
Ident 997:1000 text="sum"
LParen 1000:1001
Ident 1001:1008 text="Counter"
LParen 1008:1009
Number 1009:1010 text="0" suf="" isf=0 int=0
RParen 1010:1011
RParen 1011:1012
RParen 1012:1013
Newline 1013:1014
Ident 1018:1025 text="println"
LParen 1025:1026
Ident 1026:1029 text="sum"
LParen 1029:1030
Ident 1030:1038 text="RangeSeq"
LParen 1038:1039
Number 1039:1040 text="3" suf="" isf=0 int=3
Comma 1040:1041
Number 1042:1043 text="8" suf="" isf=0 int=8
RParen 1043:1044
RParen 1044:1045
Comma 1045:1046
Ident 1047:1050 text="sum"
LParen 1050:1051
Ident 1051:1059 text="RangeSeq"
LParen 1059:1060
Number 1060:1061 text="0" suf="" isf=0 int=0
Comma 1061:1062
Number 1063:1064 text="1" suf="" isf=0 int=1
RParen 1064:1065
RParen 1065:1066
RParen 1066:1067
Newline 1067:1068
Ident 1072:1079 text="println"
LParen 1079:1080
Ident 1080:1083 text="sum"
LParen 1083:1084
Ident 1084:1092 text="SubRange"
LParen 1092:1093
Number 1093:1094 text="3" suf="" isf=0 int=3
Comma 1094:1095
Number 1096:1097 text="7" suf="" isf=0 int=7
RParen 1097:1098
RParen 1098:1099
RParen 1099:1100
Newline 1100:1101
Newline 1101:1102
Let 1106:1109
Ident 1110:1112 text="it"
Colon 1112:1113
Ident 1114:1122 text="Iterable"
Lt 1122:1123
Ident 1123:1126 text="int"
Gt 1126:1127
Assign 1128:1129
Ident 1130:1137 text="Counter"
LParen 1137:1138
Number 1138:1139 text="4" suf="" isf=0 int=4
RParen 1139:1140
Newline 1140:1141
Var 1145:1148
Ident 1149:1156 text="via_var"
Assign 1157:1158
Number 1159:1160 text="0" suf="" isf=0 int=0
Newline 1160:1161
For 1165:1168
LParen 1169:1170
Ident 1170:1171 text="x"
In 1172:1174
Ident 1175:1177 text="it"
RParen 1177:1178
LBrace 1179:1180
Newline 1180:1181
Ident 1189:1196 text="via_var"
Assign 1197:1198
Ident 1199:1206 text="via_var"
Plus 1207:1208
Ident 1209:1210 text="x"
Newline 1210:1211
RBrace 1215:1216
Newline 1216:1217
Ident 1221:1228 text="println"
LParen 1228:1229
Ident 1229:1236 text="via_var"
RParen 1236:1237
Newline 1237:1238
Newline 1238:1239
Var 1243:1246
Ident 1247:1252 text="broke"
Assign 1253:1254
Number 1255:1256 text="0" suf="" isf=0 int=0
Newline 1256:1257
For 1261:1264
LParen 1265:1266
Ident 1266:1267 text="x"
In 1268:1270
Ident 1271:1279 text="RangeSeq"
LParen 1279:1280
Number 1280:1281 text="0" suf="" isf=0 int=0
Comma 1281:1282
Number 1283:1285 text="10" suf="" isf=0 int=10
RParen 1285:1286
RParen 1286:1287
LBrace 1288:1289
Newline 1289:1290
If 1298:1300
LParen 1301:1302
Ident 1302:1303 text="x"
Ge 1304:1306
Number 1307:1308 text="4" suf="" isf=0 int=4
RParen 1308:1309
LBrace 1310:1311
Newline 1311:1312
Break 1324:1329
Newline 1329:1330
RBrace 1338:1339
Newline 1339:1340
Ident 1348:1353 text="broke"
Assign 1354:1355
Ident 1356:1361 text="broke"
Plus 1362:1363
Ident 1364:1365 text="x"
Newline 1365:1366
RBrace 1370:1371
Newline 1371:1372
Ident 1376:1383 text="println"
LParen 1383:1384
Ident 1384:1389 text="broke"
RParen 1389:1390
Newline 1390:1391
Newline 1391:1392
Var 1396:1399
Ident 1400:1407 text="skipped"
Assign 1408:1409
Number 1410:1411 text="0" suf="" isf=0 int=0
Newline 1411:1412
For 1416:1419
LParen 1420:1421
Ident 1421:1422 text="x"
In 1423:1425
Ident 1426:1434 text="RangeSeq"
LParen 1434:1435
Number 1435:1436 text="0" suf="" isf=0 int=0
Comma 1436:1437
Number 1438:1439 text="6" suf="" isf=0 int=6
RParen 1439:1440
RParen 1440:1441
LBrace 1442:1443
Newline 1443:1444
If 1452:1454
LParen 1455:1456
Ident 1456:1457 text="x"
Percent 1458:1459
Number 1460:1461 text="2" suf="" isf=0 int=2
EqEq 1462:1464
Number 1465:1466 text="1" suf="" isf=0 int=1
RParen 1466:1467
LBrace 1468:1469
Newline 1469:1470
Continue 1482:1490
Newline 1490:1491
RBrace 1499:1500
Newline 1500:1501
Ident 1509:1516 text="skipped"
Assign 1517:1518
Ident 1519:1526 text="skipped"
Plus 1527:1528
Ident 1529:1530 text="x"
Newline 1530:1531
RBrace 1535:1536
Newline 1536:1537
Ident 1541:1548 text="println"
LParen 1548:1549
Ident 1549:1556 text="skipped"
RParen 1556:1557
Newline 1557:1558
Newline 1558:1559
Var 1563:1566
Ident 1567:1571 text="iter"
Colon 1571:1572
Ident 1573:1581 text="Iterator"
Lt 1581:1582
Ident 1582:1585 text="int"
Gt 1585:1586
Assign 1587:1588
Ident 1589:1597 text="RangeSeq"
LParen 1597:1598
Number 1598:1599 text="1" suf="" isf=0 int=1
Comma 1599:1600
Number 1601:1602 text="4" suf="" isf=0 int=4
RParen 1602:1603
Dot 1603:1604
Ident 1604:1612 text="iterator"
LParen 1612:1613
RParen 1613:1614
Newline 1614:1615
Ident 1619:1626 text="println"
LParen 1626:1627
Ident 1627:1636 text="nextOrNeg"
LParen 1636:1637
Ident 1637:1641 text="iter"
RParen 1641:1642
Comma 1642:1643
Ident 1644:1653 text="nextOrNeg"
LParen 1653:1654
Ident 1654:1658 text="iter"
RParen 1658:1659
Comma 1659:1660
Ident 1661:1670 text="nextOrNeg"
LParen 1670:1671
Ident 1671:1675 text="iter"
RParen 1675:1676
Comma 1676:1677
Ident 1678:1687 text="nextOrNeg"
LParen 1687:1688
Ident 1688:1692 text="iter"
RParen 1692:1693
RParen 1693:1694
Newline 1694:1695
RBrace 1695:1696
Eof 1696:1696
