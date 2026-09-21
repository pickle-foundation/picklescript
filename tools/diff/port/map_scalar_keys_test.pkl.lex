Ident 0:4 text="test"
Fn 5:7
Ident 8:43 text="int_keys_literal_read_and_overwrite"
LParen 43:44
RParen 44:45
LBrace 46:47
Newline 47:48
Let 52:55
Ident 56:57 text="m"
Assign 58:59
LBrace 60:61
Number 61:63 text="10" suf="" isf=0 int=10
Colon 63:64
Str 65:70 T("ten")
Comma 70:71
Number 72:74 text="20" suf="" isf=0 int=20
Colon 74:75
Str 76:84 T("twenty")
RBrace 84:85
Newline 85:86
Ident 90:96 text="expect"
LParen 96:97
Ident 97:98 text="m"
LBracket 98:99
Number 99:101 text="10" suf="" isf=0 int=10
RBracket 101:102
RParen 102:103
Dot 103:104
Ident 104:108 text="toBe"
LParen 108:109
Str 109:114 T("ten")
RParen 114:115
Newline 115:116
Ident 120:126 text="expect"
LParen 126:127
Ident 127:128 text="m"
LBracket 128:129
Number 129:131 text="20" suf="" isf=0 int=20
RBracket 131:132
RParen 132:133
Dot 133:134
Ident 134:138 text="toBe"
LParen 138:139
Str 139:147 T("twenty")
RParen 147:148
Newline 148:149
Ident 153:154 text="m"
LBracket 154:155
Number 155:157 text="10" suf="" isf=0 int=10
RBracket 157:158
Assign 159:160
Str 161:166 T("TEN")
Newline 166:167
Ident 171:177 text="expect"
LParen 177:178
Ident 178:179 text="m"
LBracket 179:180
Number 180:182 text="10" suf="" isf=0 int=10
RBracket 182:183
RParen 183:184
Dot 184:185
Ident 185:189 text="toBe"
LParen 189:190
Str 190:195 T("TEN")
RParen 195:196
Newline 196:197
Ident 201:207 text="expect"
LParen 207:208
Ident 208:211 text="len"
LParen 211:212
Ident 212:213 text="m"
RParen 213:214
RParen 214:215
Dot 215:216
Ident 216:220 text="toBe"
LParen 220:221
Number 221:222 text="2" suf="" isf=0 int=2
RParen 222:223
Newline 223:224
RBrace 224:225
Newline 225:226
Newline 226:227
Ident 227:231 text="test"
Fn 232:234
Ident 235:257 text="int_map_index_defaults"
LParen 257:258
RParen 258:259
LBrace 260:261
Newline 261:262
Let 266:269
Ident 270:272 text="vi"
Assign 273:274
LBrace 275:276
Number 276:277 text="5" suf="" isf=0 int=5
Colon 277:278
Number 279:280 text="7" suf="" isf=0 int=7
RBrace 280:281
Newline 281:282
Ident 286:292 text="expect"
LParen 292:293
Ident 293:295 text="vi"
LBracket 295:296
Number 296:297 text="9" suf="" isf=0 int=9
RBracket 297:298
RParen 298:299
Dot 299:300
Ident 300:304 text="toBe"
LParen 304:305
Number 305:306 text="0" suf="" isf=0 int=0
RParen 306:307
Newline 307:308
Let 312:315
Ident 316:318 text="vs"
Assign 319:320
LBrace 321:322
Number 322:323 text="5" suf="" isf=0 int=5
Colon 323:324
Str 325:331 T("five")
RBrace 331:332
Newline 332:333
Ident 337:343 text="expect"
LParen 343:344
Ident 344:346 text="vs"
LBracket 346:347
Number 347:348 text="9" suf="" isf=0 int=9
RBracket 348:349
RParen 349:350
Dot 350:351
Ident 351:355 text="toBe"
LParen 355:356
Str 356:358
RParen 358:359
Newline 359:360
RBrace 360:361
Newline 361:362
Newline 362:363
Ident 363:367 text="test"
Fn 368:370
Ident 371:386 text="int_map_methods"
LParen 386:387
RParen 387:388
LBrace 389:390
Newline 390:391
Let 395:398
Ident 399:400 text="m"
Assign 401:402
LBrace 403:404
Number 404:405 text="1" suf="" isf=0 int=1
Colon 405:406
Str 407:412 T("one")
Comma 412:413
Number 414:415 text="2" suf="" isf=0 int=2
Colon 415:416
Str 417:422 T("two")
RBrace 422:423
Newline 423:424
Ident 428:434 text="expect"
LParen 434:435
Ident 435:436 text="m"
Dot 436:437
Ident 437:440 text="has"
LParen 440:441
Number 441:442 text="1" suf="" isf=0 int=1
RParen 442:443
RParen 443:444
Dot 444:445
Ident 445:449 text="toBe"
LParen 449:450
True 450:454
RParen 454:455
Newline 455:456
Ident 460:466 text="expect"
LParen 466:467
Ident 467:468 text="m"
Dot 468:469
Ident 469:472 text="has"
LParen 472:473
Number 473:474 text="3" suf="" isf=0 int=3
RParen 474:475
RParen 475:476
Dot 476:477
Ident 477:481 text="toBe"
LParen 481:482
False 482:487
RParen 487:488
Newline 488:489
If 493:495
LParen 496:497
Let 497:500
Ident 501:505 text="some"
LParen 505:506
Ident 506:507 text="v"
RParen 507:508
Assign 509:510
Ident 511:512 text="m"
Dot 512:513
Get 513:516
LParen 516:517
Number 517:518 text="1" suf="" isf=0 int=1
RParen 518:519
RParen 519:520
LBrace 521:522
Newline 522:523
Ident 531:537 text="expect"
LParen 537:538
Ident 538:539 text="v"
RParen 539:540
Dot 540:541
Ident 541:545 text="toBe"
LParen 545:546
Str 546:551 T("one")
RParen 551:552
Newline 552:553
RBrace 557:558
Else 559:563
LBrace 564:565
Newline 565:566
Ident 574:580 text="expect"
LParen 580:581
Str 581:593 T("unexpected")
RParen 593:594
Dot 594:595
Ident 595:599 text="toBe"
LParen 599:600
Str 600:654 T("int_map_methods: get returned none for a present key")
RParen 654:655
Newline 655:656
RBrace 660:661
Newline 661:662
Let 666:669
Ident 670:671 text="v"
Assign 672:673
Ident 674:675 text="m"
Dot 675:676
Get 676:679
LParen 679:680
Number 680:681 text="9" suf="" isf=0 int=9
RParen 681:682
QuestionQuestion 683:685
Str 686:695 T("missing")
Newline 695:696
Ident 700:706 text="expect"
LParen 706:707
Ident 707:708 text="v"
RParen 708:709
Dot 709:710
Ident 710:714 text="toBe"
LParen 714:715
Str 715:724 T("missing")
RParen 724:725
Newline 725:726
Ident 730:736 text="expect"
LParen 736:737
Ident 737:740 text="len"
LParen 740:741
Ident 741:742 text="m"
Dot 742:743
Ident 743:747 text="keys"
LParen 747:748
RParen 748:749
RParen 749:750
RParen 750:751
Dot 751:752
Ident 752:756 text="toBe"
LParen 756:757
Number 757:758 text="2" suf="" isf=0 int=2
RParen 758:759
Newline 759:760
Ident 764:770 text="expect"
LParen 770:771
Ident 771:774 text="len"
LParen 774:775
Ident 775:776 text="m"
Dot 776:777
Ident 777:783 text="values"
LParen 783:784
RParen 784:785
RParen 785:786
RParen 786:787
Dot 787:788
Ident 788:792 text="toBe"
LParen 792:793
Number 793:794 text="2" suf="" isf=0 int=2
RParen 794:795
Newline 795:796
If 800:802
LParen 803:804
Let 804:807
Ident 808:812 text="some"
LParen 812:813
Ident 813:814 text="r"
RParen 814:815
Assign 816:817
Ident 818:819 text="m"
Dot 819:820
Ident 820:826 text="remove"
LParen 826:827
Number 827:828 text="2" suf="" isf=0 int=2
RParen 828:829
RParen 829:830
LBrace 831:832
Newline 832:833
Ident 841:847 text="expect"
LParen 847:848
Ident 848:849 text="r"
RParen 849:850
Dot 850:851
Ident 851:855 text="toBe"
LParen 855:856
Str 856:861 T("two")
RParen 861:862
Newline 862:863
RBrace 867:868
Else 869:873
LBrace 874:875
Newline 875:876
Ident 884:890 text="expect"
LParen 890:891
Str 891:903 T("unexpected")
RParen 903:904
Dot 904:905
Ident 905:909 text="toBe"
LParen 909:910
Str 910:967 T("int_map_methods: remove returned none for a present key")
RParen 967:968
Newline 968:969
RBrace 973:974
Newline 974:975
Ident 979:985 text="expect"
LParen 985:986
Ident 986:987 text="m"
Dot 987:988
Ident 988:991 text="has"
LParen 991:992
Number 992:993 text="2" suf="" isf=0 int=2
RParen 993:994
RParen 994:995
Dot 995:996
Ident 996:1000 text="toBe"
LParen 1000:1001
False 1001:1006
RParen 1006:1007
Newline 1007:1008
Ident 1012:1018 text="expect"
LParen 1018:1019
Ident 1019:1022 text="len"
LParen 1022:1023
Ident 1023:1024 text="m"
RParen 1024:1025
RParen 1025:1026
Dot 1026:1027
Ident 1027:1031 text="toBe"
LParen 1031:1032
Number 1032:1033 text="1" suf="" isf=0 int=1
RParen 1033:1034
Newline 1034:1035
RBrace 1035:1036
Newline 1036:1037
Newline 1037:1038
Ident 1038:1042 text="test"
Fn 1043:1045
Ident 1046:1070 text="int_keys_iterate_entries"
LParen 1070:1071
RParen 1071:1072
LBrace 1073:1074
Newline 1074:1075
Let 1079:1082
Ident 1083:1084 text="m"
Assign 1085:1086
LBrace 1087:1088
Number 1088:1089 text="2" suf="" isf=0 int=2
Colon 1089:1090
Number 1091:1093 text="20" suf="" isf=0 int=20
Comma 1093:1094
Number 1095:1096 text="3" suf="" isf=0 int=3
Colon 1096:1097
Number 1098:1100 text="30" suf="" isf=0 int=30
Comma 1100:1101
Number 1102:1103 text="5" suf="" isf=0 int=5
Colon 1103:1104
Number 1105:1107 text="50" suf="" isf=0 int=50
RBrace 1107:1108
Newline 1108:1109
Var 1113:1116
Ident 1117:1122 text="sum_k"
Assign 1123:1124
Number 1125:1126 text="0" suf="" isf=0 int=0
Newline 1126:1127
Var 1131:1134
Ident 1135:1140 text="sum_v"
Assign 1141:1142
Number 1143:1144 text="0" suf="" isf=0 int=0
Newline 1144:1145
Var 1149:1152
Ident 1153:1154 text="n"
Assign 1155:1156
Number 1157:1158 text="0" suf="" isf=0 int=0
Newline 1158:1159
For 1163:1166
LParen 1167:1168
LParen 1168:1169
Ident 1169:1170 text="k"
Comma 1170:1171
Ident 1172:1173 text="v"
RParen 1173:1174
In 1175:1177
Ident 1178:1179 text="m"
RParen 1179:1180
LBrace 1181:1182
Newline 1182:1183
Ident 1191:1192 text="n"
PlusEq 1193:1195
Number 1196:1197 text="1" suf="" isf=0 int=1
Newline 1197:1198
Ident 1206:1211 text="sum_k"
PlusEq 1212:1214
Ident 1215:1216 text="k"
Newline 1216:1217
Ident 1225:1230 text="sum_v"
PlusEq 1231:1233
Ident 1234:1235 text="v"
Newline 1235:1236
Ident 1244:1250 text="expect"
LParen 1250:1251
Ident 1251:1252 text="m"
LBracket 1252:1253
Ident 1253:1254 text="k"
RBracket 1254:1255
RParen 1255:1256
Dot 1256:1257
Ident 1257:1261 text="toBe"
LParen 1261:1262
Ident 1262:1263 text="v"
RParen 1263:1264
Newline 1264:1265
RBrace 1269:1270
Newline 1270:1271
Ident 1275:1281 text="expect"
LParen 1281:1282
Ident 1282:1283 text="n"
RParen 1283:1284
Dot 1284:1285
Ident 1285:1289 text="toBe"
LParen 1289:1290
Number 1290:1291 text="3" suf="" isf=0 int=3
RParen 1291:1292
Newline 1292:1293
Ident 1297:1303 text="expect"
LParen 1303:1304
Ident 1304:1309 text="sum_k"
RParen 1309:1310
Dot 1310:1311
Ident 1311:1315 text="toBe"
LParen 1315:1316
Number 1316:1318 text="10" suf="" isf=0 int=10
RParen 1318:1319
Newline 1319:1320
Ident 1324:1330 text="expect"
LParen 1330:1331
Ident 1331:1336 text="sum_v"
RParen 1336:1337
Dot 1337:1338
Ident 1338:1342 text="toBe"
LParen 1342:1343
Number 1343:1346 text="100" suf="" isf=0 int=100
RParen 1346:1347
Newline 1347:1348
RBrace 1348:1349
Newline 1349:1350
Newline 1350:1351
Ident 1351:1355 text="test"
Fn 1356:1358
Ident 1359:1397 text="int_keys_iterate_plain_values_and_keys"
LParen 1397:1398
RParen 1398:1399
LBrace 1400:1401
Newline 1401:1402
Let 1406:1409
Ident 1410:1411 text="m"
Assign 1412:1413
LBrace 1414:1415
Number 1415:1416 text="4" suf="" isf=0 int=4
Colon 1416:1417
Number 1418:1420 text="40" suf="" isf=0 int=40
Comma 1420:1421
Number 1422:1423 text="6" suf="" isf=0 int=6
Colon 1423:1424
Number 1425:1427 text="60" suf="" isf=0 int=60
RBrace 1427:1428
Newline 1428:1429
Var 1433:1436
Ident 1437:1442 text="sum_v"
Assign 1443:1444
Number 1445:1446 text="0" suf="" isf=0 int=0
Newline 1446:1447
For 1451:1454
LParen 1455:1456
Ident 1456:1457 text="v"
In 1458:1460
Ident 1461:1462 text="m"
RParen 1462:1463
LBrace 1464:1465
Newline 1465:1466
Ident 1474:1479 text="sum_v"
PlusEq 1480:1482
Ident 1483:1484 text="v"
Newline 1484:1485
RBrace 1489:1490
Newline 1490:1491
Ident 1495:1501 text="expect"
LParen 1501:1502
Ident 1502:1507 text="sum_v"
RParen 1507:1508
Dot 1508:1509
Ident 1509:1513 text="toBe"
LParen 1513:1514
Number 1514:1517 text="100" suf="" isf=0 int=100
RParen 1517:1518
Newline 1518:1519
Var 1523:1526
Ident 1527:1532 text="sum_k"
Assign 1533:1534
Number 1535:1536 text="0" suf="" isf=0 int=0
Newline 1536:1537
For 1541:1544
LParen 1545:1546
Ident 1546:1547 text="k"
In 1548:1550
Ident 1551:1552 text="m"
Dot 1552:1553
Ident 1553:1557 text="keys"
LParen 1557:1558
RParen 1558:1559
RParen 1559:1560
LBrace 1561:1562
Newline 1562:1563
Ident 1571:1576 text="sum_k"
PlusEq 1577:1579
Ident 1580:1581 text="k"
Newline 1581:1582
RBrace 1586:1587
Newline 1587:1588
Ident 1592:1598 text="expect"
LParen 1598:1599
Ident 1599:1604 text="sum_k"
RParen 1604:1605
Dot 1605:1606
Ident 1606:1610 text="toBe"
LParen 1610:1611
Number 1611:1613 text="10" suf="" isf=0 int=10
RParen 1613:1614
Newline 1614:1615
RBrace 1615:1616
Newline 1616:1617
Newline 1617:1618
Ident 1618:1622 text="test"
Fn 1623:1625
Ident 1626:1636 text="float_keys"
LParen 1636:1637
RParen 1637:1638
LBrace 1639:1640
Newline 1640:1641
Let 1645:1648
Ident 1649:1650 text="m"
Assign 1651:1652
LBrace 1653:1654
Number 1654:1657 text="1.5" suf="" isf=1 int=-
Colon 1657:1658
Str 1659:1675 T("one point five")
Comma 1675:1676
Number 1677:1680 text="2.5" suf="" isf=1 int=-
Colon 1680:1681
Str 1682:1698 T("two point five")
RBrace 1698:1699
Newline 1699:1700
Ident 1704:1710 text="expect"
LParen 1710:1711
Ident 1711:1712 text="m"
LBracket 1712:1713
Number 1713:1716 text="1.5" suf="" isf=1 int=-
RBracket 1716:1717
RParen 1717:1718
Dot 1718:1719
Ident 1719:1723 text="toBe"
LParen 1723:1724
Str 1724:1740 T("one point five")
RParen 1740:1741
Newline 1741:1742
Ident 1746:1752 text="expect"
LParen 1752:1753
Ident 1753:1754 text="m"
Dot 1754:1755
Ident 1755:1758 text="has"
LParen 1758:1759
Number 1759:1762 text="2.5" suf="" isf=1 int=-
RParen 1762:1763
RParen 1763:1764
Dot 1764:1765
Ident 1765:1769 text="toBe"
LParen 1769:1770
True 1770:1774
RParen 1774:1775
Newline 1775:1776
Ident 1780:1786 text="expect"
LParen 1786:1787
Ident 1787:1788 text="m"
Dot 1788:1789
Ident 1789:1792 text="has"
LParen 1792:1793
Number 1793:1797 text="1.25" suf="" isf=1 int=-
RParen 1797:1798
RParen 1798:1799
Dot 1799:1800
Ident 1800:1804 text="toBe"
LParen 1804:1805
False 1805:1810
RParen 1810:1811
Newline 1811:1812
Ident 1816:1817 text="m"
LBracket 1817:1818
Number 1818:1821 text="3.5" suf="" isf=1 int=-
RBracket 1821:1822
Assign 1823:1824
Str 1825:1843 T("three point five")
Newline 1843:1844
Ident 1848:1854 text="expect"
LParen 1854:1855
Ident 1855:1856 text="m"
LBracket 1856:1857
Number 1857:1860 text="3.5" suf="" isf=1 int=-
RBracket 1860:1861
RParen 1861:1862
Dot 1862:1863
Ident 1863:1867 text="toBe"
LParen 1867:1868
Str 1868:1886 T("three point five")
RParen 1886:1887
Newline 1887:1888
Ident 1892:1898 text="expect"
LParen 1898:1899
Ident 1899:1902 text="len"
LParen 1902:1903
Ident 1903:1904 text="m"
RParen 1904:1905
RParen 1905:1906
Dot 1906:1907
Ident 1907:1911 text="toBe"
LParen 1911:1912
Number 1912:1913 text="3" suf="" isf=0 int=3
RParen 1913:1914
Newline 1914:1915
Var 1919:1922
Ident 1923:1926 text="sum"
Assign 1927:1928
Number 1929:1932 text="0.0" suf="" isf=1 int=-
Newline 1932:1933
For 1937:1940
LParen 1941:1942
LParen 1942:1943
Ident 1943:1944 text="k"
Comma 1944:1945
Ident 1946:1947 text="v"
RParen 1947:1948
In 1949:1951
Ident 1952:1953 text="m"
RParen 1953:1954
LBrace 1955:1956
Newline 1956:1957
Ident 1965:1968 text="sum"
PlusEq 1969:1971
Ident 1972:1973 text="k"
Newline 1973:1974
Ident 1982:1988 text="expect"
LParen 1988:1989
Ident 1989:1992 text="len"
LParen 1992:1993
Ident 1993:1994 text="v"
RParen 1994:1995
Gt 1996:1997
Number 1998:1999 text="0" suf="" isf=0 int=0
OrOr 2000:2002
Ident 2003:2006 text="len"
LParen 2006:2007
Ident 2007:2008 text="v"
RParen 2008:2009
EqEq 2010:2012
Number 2013:2014 text="0" suf="" isf=0 int=0
RParen 2014:2015
Dot 2015:2016
Ident 2016:2020 text="toBe"
LParen 2020:2021
True 2021:2025
RParen 2025:2026
Newline 2026:2027
RBrace 2031:2032
Newline 2032:2033
Ident 2037:2043 text="expect"
LParen 2043:2044
Ident 2044:2047 text="sum"
RParen 2047:2048
Dot 2048:2049
Ident 2049:2053 text="toBe"
LParen 2053:2054
Number 2054:2057 text="7.5" suf="" isf=1 int=-
RParen 2057:2058
Newline 2058:2059
RBrace 2059:2060
Newline 2060:2061
Newline 2061:2062
Ident 2062:2066 text="test"
Fn 2067:2069
Ident 2070:2084 text="bool_char_keys"
LParen 2084:2085
RParen 2085:2086
LBrace 2087:2088
Newline 2088:2089
Let 2093:2096
Ident 2097:2099 text="mb"
Assign 2100:2101
LBrace 2102:2103
True 2103:2107
Colon 2107:2108
Str 2109:2114 T("yes")
Comma 2114:2115
False 2116:2121
Colon 2121:2122
Str 2123:2127 T("no")
RBrace 2127:2128
Newline 2128:2129
Ident 2133:2139 text="expect"
LParen 2139:2140
Ident 2140:2142 text="mb"
LBracket 2142:2143
True 2143:2147
RBracket 2147:2148
RParen 2148:2149
Dot 2149:2150
Ident 2150:2154 text="toBe"
LParen 2154:2155
Str 2155:2160 T("yes")
RParen 2160:2161
Newline 2161:2162
Ident 2166:2172 text="expect"
LParen 2172:2173
Ident 2173:2175 text="mb"
LBracket 2175:2176
False 2176:2181
RBracket 2181:2182
RParen 2182:2183
Dot 2183:2184
Ident 2184:2188 text="toBe"
LParen 2188:2189
Str 2189:2193 T("no")
RParen 2193:2194
Newline 2194:2195
Ident 2199:2205 text="expect"
LParen 2205:2206
Ident 2206:2208 text="mb"
Dot 2208:2209
Ident 2209:2212 text="has"
LParen 2212:2213
True 2213:2217
RParen 2217:2218
RParen 2218:2219
Dot 2219:2220
Ident 2220:2224 text="toBe"
LParen 2224:2225
True 2225:2229
RParen 2229:2230
Newline 2230:2231
Let 2235:2238
Ident 2239:2241 text="mc"
Assign 2242:2243
LBrace 2244:2245
Char 2245:2248 ch='a' int=97
Colon 2248:2249
Number 2250:2251 text="1" suf="" isf=0 int=1
Comma 2251:2252
Char 2253:2256 ch='z' int=122
Colon 2256:2257
Number 2258:2259 text="2" suf="" isf=0 int=2
RBrace 2259:2260
Newline 2260:2261
Ident 2265:2271 text="expect"
LParen 2271:2272
Ident 2272:2274 text="mc"
LBracket 2274:2275
Char 2275:2278 ch='a' int=97
RBracket 2278:2279
RParen 2279:2280
Dot 2280:2281
Ident 2281:2285 text="toBe"
LParen 2285:2286
Number 2286:2287 text="1" suf="" isf=0 int=1
RParen 2287:2288
Newline 2288:2289
Ident 2293:2299 text="expect"
LParen 2299:2300
Ident 2300:2302 text="mc"
LBracket 2302:2303
Char 2303:2306 ch='z' int=122
RBracket 2306:2307
RParen 2307:2308
Dot 2308:2309
Ident 2309:2313 text="toBe"
LParen 2313:2314
Number 2314:2315 text="2" suf="" isf=0 int=2
RParen 2315:2316
Newline 2316:2317
Ident 2321:2327 text="expect"
LParen 2327:2328
Ident 2328:2330 text="mc"
Dot 2330:2331
Ident 2331:2334 text="has"
LParen 2334:2335
Char 2335:2338 ch='m' int=109
RParen 2338:2339
RParen 2339:2340
Dot 2340:2341
Ident 2341:2345 text="toBe"
LParen 2345:2346
False 2346:2351
RParen 2351:2352
Newline 2352:2353
RBrace 2353:2354
Newline 2354:2355
Newline 2355:2356
Ident 2356:2360 text="test"
Fn 2361:2363
Ident 2364:2391 text="byte_keys_from_string_index"
LParen 2391:2392
RParen 2392:2393
LBrace 2394:2395
Newline 2395:2396
Let 2400:2403
Ident 2404:2405 text="s"
Assign 2406:2407
Str 2408:2412 T("hi")
Newline 2412:2413
Let 2417:2420
Ident 2421:2422 text="m"
Assign 2423:2424
LBrace 2425:2426
Ident 2426:2427 text="s"
LBracket 2427:2428
Number 2428:2429 text="0" suf="" isf=0 int=0
RBracket 2429:2430
Colon 2430:2431
Str 2432:2439 T("first")
Comma 2439:2440
Ident 2441:2442 text="s"
LBracket 2442:2443
Number 2443:2444 text="1" suf="" isf=0 int=1
RBracket 2444:2445
Colon 2445:2446
Str 2447:2455 T("second")
RBrace 2455:2456
Newline 2456:2457
Ident 2461:2467 text="expect"
LParen 2467:2468
Ident 2468:2469 text="m"
LBracket 2469:2470
Ident 2470:2471 text="s"
LBracket 2471:2472
Number 2472:2473 text="0" suf="" isf=0 int=0
RBracket 2473:2474
RBracket 2474:2475
RParen 2475:2476
Dot 2476:2477
Ident 2477:2481 text="toBe"
LParen 2481:2482
Str 2482:2489 T("first")
RParen 2489:2490
Newline 2490:2491
Ident 2495:2501 text="expect"
LParen 2501:2502
Ident 2502:2503 text="m"
LBracket 2503:2504
Ident 2504:2505 text="s"
LBracket 2505:2506
Number 2506:2507 text="1" suf="" isf=0 int=1
RBracket 2507:2508
RBracket 2508:2509
RParen 2509:2510
Dot 2510:2511
Ident 2511:2515 text="toBe"
LParen 2515:2516
Str 2516:2524 T("second")
RParen 2524:2525
Newline 2525:2526
Newline 2594:2595
Ident 2599:2605 text="expect"
LParen 2605:2606
Ident 2606:2607 text="m"
LBracket 2607:2608
Number 2608:2611 text="104" suf="" isf=0 int=104
RBracket 2611:2612
RParen 2612:2613
Dot 2613:2614
Ident 2614:2618 text="toBe"
LParen 2618:2619
Str 2619:2626 T("first")
RParen 2626:2627
Newline 2627:2628
Ident 2632:2638 text="expect"
LParen 2638:2639
Ident 2639:2640 text="m"
LBracket 2640:2641
Number 2641:2644 text="105" suf="" isf=0 int=105
RBracket 2644:2645
RParen 2645:2646
Dot 2646:2647
Ident 2647:2651 text="toBe"
LParen 2651:2652
Str 2652:2660 T("second")
RParen 2660:2661
Newline 2661:2662
RBrace 2662:2663
Newline 2663:2664
Newline 2664:2665
Ident 2665:2669 text="test"
Fn 2670:2672
Ident 2673:2692 text="large_int_map_grows"
LParen 2692:2693
RParen 2693:2694
LBrace 2695:2696
Newline 2696:2697
Var 2701:2704
Ident 2705:2706 text="m"
Assign 2707:2708
LBrace 2709:2710
Number 2710:2711 text="0" suf="" isf=0 int=0
Colon 2711:2712
Str 2713:2719 T("seed")
RBrace 2719:2720
Newline 2720:2721
For 2725:2728
LParen 2729:2730
Ident 2730:2731 text="i"
In 2732:2734
Number 2735:2736 text="1" suf="" isf=0 int=1
Range 2736:2738
Number 2738:2741 text="300" suf="" isf=0 int=300
RParen 2741:2742
LBrace 2743:2744
Newline 2744:2745
Ident 2753:2754 text="m"
LBracket 2754:2755
Ident 2755:2756 text="i"
RBracket 2756:2757
Assign 2758:2759
Str 2760:2770 E[Ident 2762:2763 text="i"; Eof 2764:2764] T(" pair")
Newline 2770:2771
RBrace 2775:2776
Newline 2776:2777
Ident 2781:2787 text="expect"
LParen 2787:2788
Ident 2788:2791 text="len"
LParen 2791:2792
Ident 2792:2793 text="m"
RParen 2793:2794
RParen 2794:2795
Dot 2795:2796
Ident 2796:2800 text="toBe"
LParen 2800:2801
Number 2801:2804 text="300" suf="" isf=0 int=300
RParen 2804:2805
Newline 2805:2806
Var 2810:2813
Ident 2814:2815 text="n"
Assign 2816:2817
Number 2818:2819 text="0" suf="" isf=0 int=0
Newline 2819:2820
For 2824:2827
LParen 2828:2829
LParen 2829:2830
Ident 2830:2831 text="k"
Comma 2831:2832
Ident 2833:2834 text="v"
RParen 2834:2835
In 2836:2838
Ident 2839:2840 text="m"
RParen 2840:2841
LBrace 2842:2843
Newline 2843:2844
If 2852:2854
LParen 2855:2856
Ident 2856:2857 text="m"
LBracket 2857:2858
Ident 2858:2859 text="k"
RBracket 2859:2860
EqEq 2861:2863
Ident 2864:2865 text="v"
RParen 2865:2866
LBrace 2867:2868
Newline 2868:2869
Ident 2881:2882 text="n"
PlusEq 2883:2885
Number 2886:2887 text="1" suf="" isf=0 int=1
Newline 2887:2888
RBrace 2896:2897
Newline 2897:2898
RBrace 2902:2903
Newline 2903:2904
Ident 2908:2914 text="expect"
LParen 2914:2915
Ident 2915:2916 text="n"
RParen 2916:2917
Dot 2917:2918
Ident 2918:2922 text="toBe"
LParen 2922:2923
Number 2923:2926 text="300" suf="" isf=0 int=300
RParen 2926:2927
Newline 2927:2928
RBrace 2928:2929
Eof 2929:2929
