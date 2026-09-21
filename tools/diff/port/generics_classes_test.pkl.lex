Class 0:5
Ident 6:9 text="Box"
Lt 9:10
Ident 10:11 text="T"
Gt 11:12
LBrace 13:14
Newline 14:15
Var 19:22
Ident 23:28 text="value"
Colon 28:29
Ident 30:31 text="T"
Newline 31:32
Newline 32:33
Fn 37:39
Ident 40:44 text="read"
LParen 44:45
RParen 45:46
Arrow 47:49
Ident 50:51 text="T"
LBrace 52:53
Newline 53:54
This 62:66
Dot 66:67
Ident 67:72 text="value"
Newline 72:73
RBrace 77:78
Newline 78:79
Newline 79:80
Fn 84:86
Ident 87:92 text="write"
LParen 92:93
Ident 93:101 text="newValue"
Colon 101:102
Ident 103:104 text="T"
RParen 104:105
LBrace 106:107
Newline 107:108
This 116:120
Dot 120:121
Ident 121:126 text="value"
Assign 127:128
Ident 129:137 text="newValue"
Newline 137:138
RBrace 142:143
Newline 143:144
RBrace 144:145
Newline 145:146
Newline 146:147
Struct 147:153
Ident 154:158 text="Pair"
Lt 158:159
Ident 159:160 text="A"
Comma 160:161
Ident 162:163 text="B"
Gt 163:164
LBrace 165:166
Newline 166:167
Var 171:174
Ident 175:180 text="first"
Colon 180:181
Ident 182:183 text="A"
Newline 183:184
Var 188:191
Ident 192:198 text="second"
Colon 198:199
Ident 200:201 text="B"
Newline 201:202
RBrace 202:203
Newline 203:204
Newline 204:205
Class 205:210
Ident 211:218 text="Counter"
Lt 218:219
Ident 219:220 text="T"
Gt 220:221
LBrace 222:223
Newline 223:224
Var 228:231
Ident 232:237 text="count"
Colon 237:238
Ident 239:242 text="int"
Assign 243:244
Number 245:246 text="0" suf="" isf=0 int=0
Newline 246:247
Var 251:254
Ident 255:259 text="item"
Colon 259:260
Ident 261:262 text="T"
Newline 262:263
Newline 263:264
Fn 268:270
Ident 271:275 text="bump"
LParen 275:276
RParen 276:277
Arrow 278:280
Ident 281:284 text="int"
LBrace 285:286
Newline 286:287
This 295:299
Dot 299:300
Ident 300:305 text="count"
Assign 306:307
This 308:312
Dot 312:313
Ident 313:318 text="count"
Plus 319:320
Number 321:322 text="1" suf="" isf=0 int=1
Newline 322:323
This 331:335
Dot 335:336
Ident 336:341 text="count"
Newline 341:342
RBrace 346:347
Newline 347:348
RBrace 348:349
Newline 349:350
Newline 350:351
Class 351:356
Ident 357:364 text="ListBox"
Lt 364:365
Ident 365:366 text="T"
Gt 366:367
LBrace 368:369
Newline 369:370
Var 374:377
Ident 378:383 text="items"
Colon 383:384
Ident 385:389 text="List"
Lt 389:390
Ident 390:391 text="T"
Gt 391:392
Newline 392:393
Var 397:400
Ident 401:405 text="name"
Colon 405:406
Ident 407:413 text="string"
Newline 413:414
Newline 414:415
Fn 419:421
Ident 422:431 text="firstItem"
LParen 431:432
Ident 432:440 text="fallback"
Colon 440:441
Ident 442:443 text="T"
RParen 443:444
Arrow 445:447
Ident 448:449 text="T"
LBrace 450:451
Newline 451:452
If 460:462
LParen 463:464
Ident 464:467 text="len"
LParen 467:468
This 468:472
Dot 472:473
Ident 473:478 text="items"
RParen 478:479
Gt 480:481
Number 482:483 text="0" suf="" isf=0 int=0
RParen 483:484
LBrace 485:486
Newline 486:487
This 499:503
Dot 503:504
Ident 504:509 text="items"
LBracket 509:510
Number 510:511 text="0" suf="" isf=0 int=0
RBracket 511:512
Newline 512:513
RBrace 521:522
Else 523:527
LBrace 528:529
Newline 529:530
Ident 542:550 text="fallback"
Newline 550:551
RBrace 559:560
Newline 560:561
RBrace 565:566
Newline 566:567
RBrace 567:568
Newline 568:569
Newline 569:570
Fn 570:572
Ident 573:578 text="unbox"
Lt 578:579
Ident 579:580 text="T"
Gt 580:581
LParen 581:582
Ident 582:583 text="b"
Colon 583:584
Ident 585:588 text="Box"
Lt 588:589
Ident 589:590 text="T"
Gt 590:591
RParen 591:592
Arrow 593:595
Ident 596:597 text="T"
LBrace 598:599
Newline 599:600
Ident 604:605 text="b"
Dot 605:606
Ident 606:610 text="read"
LParen 610:611
RParen 611:612
Newline 612:613
RBrace 613:614
Newline 614:615
Newline 615:616
Fn 616:618
Ident 619:623 text="swap"
Lt 623:624
Ident 624:625 text="T"
Gt 625:626
LParen 626:627
Ident 627:628 text="b"
Colon 628:629
Ident 630:633 text="Box"
Lt 633:634
Ident 634:635 text="T"
Gt 635:636
Comma 636:637
Ident 638:639 text="x"
Colon 639:640
Ident 641:642 text="T"
RParen 642:643
Arrow 644:646
Ident 647:648 text="T"
LBrace 649:650
Newline 650:651
Var 655:658
Ident 659:662 text="old"
Assign 663:664
Ident 665:666 text="b"
Dot 666:667
Ident 667:671 text="read"
LParen 671:672
RParen 672:673
Newline 673:674
Ident 678:679 text="b"
Dot 679:680
Ident 680:685 text="write"
LParen 685:686
Ident 686:687 text="x"
RParen 687:688
Newline 688:689
Ident 693:696 text="old"
Newline 696:697
RBrace 697:698
Newline 698:699
Newline 699:700
Fn 700:702
Ident 703:715 text="nested_boxes"
LParen 715:716
RParen 716:717
Arrow 718:720
Ident 721:724 text="int"
LBrace 725:726
Newline 726:727
Var 731:734
Ident 735:740 text="inner"
Assign 741:742
Ident 743:746 text="Box"
Lt 746:747
Ident 747:750 text="int"
Gt 750:751
LParen 751:752
Number 752:753 text="7" suf="" isf=0 int=7
RParen 753:754
Newline 754:755
Var 759:762
Ident 763:768 text="outer"
Assign 769:770
Ident 771:774 text="Box"
Lt 774:775
Ident 775:778 text="Box"
Lt 778:779
Ident 779:782 text="int"
Shr 782:784
LParen 784:785
Ident 785:790 text="inner"
RParen 790:791
Newline 791:792
Var 796:799
Ident 800:803 text="got"
Colon 803:804
Ident 805:808 text="Box"
Lt 808:809
Ident 809:812 text="int"
Gt 812:813
Assign 814:815
Ident 816:821 text="outer"
Dot 821:822
Ident 822:826 text="read"
LParen 826:827
RParen 827:828
Newline 828:829
Ident 833:836 text="got"
Dot 836:837
Ident 837:841 text="read"
LParen 841:842
RParen 842:843
Newline 843:844
RBrace 844:845
Newline 845:846
Newline 846:847
Fn 847:849
Ident 850:859 text="make_pair"
Lt 859:860
Ident 860:861 text="A"
Comma 861:862
Ident 863:864 text="B"
Gt 864:865
LParen 865:866
Ident 866:867 text="a"
Colon 867:868
Ident 869:870 text="A"
Comma 870:871
Ident 872:873 text="b"
Colon 873:874
Ident 875:876 text="B"
RParen 876:877
Arrow 878:880
Ident 881:885 text="Pair"
Lt 885:886
Ident 886:887 text="A"
Comma 887:888
Ident 889:890 text="B"
Gt 890:891
LBrace 892:893
Newline 893:894
Ident 898:902 text="Pair"
Lt 902:903
Ident 903:904 text="A"
Comma 904:905
Ident 906:907 text="B"
Gt 907:908
LParen 908:909
Ident 909:910 text="a"
Comma 910:911
Ident 912:913 text="b"
RParen 913:914
Newline 914:915
RBrace 915:916
Newline 916:917
Newline 917:918
Ident 918:922 text="test"
Fn 923:925
Ident 926:943 text="generic_class_int"
LParen 943:944
RParen 944:945
LBrace 946:947
Newline 947:948
Var 952:955
Ident 956:957 text="b"
Assign 958:959
Ident 960:963 text="Box"
Lt 963:964
Ident 964:967 text="int"
Gt 967:968
LParen 968:969
Number 969:970 text="5" suf="" isf=0 int=5
RParen 970:971
Newline 971:972
Ident 976:982 text="expect"
LParen 982:983
Ident 983:984 text="b"
Dot 984:985
Ident 985:989 text="read"
LParen 989:990
RParen 990:991
RParen 991:992
Dot 992:993
Ident 993:997 text="toBe"
LParen 997:998
Number 998:999 text="5" suf="" isf=0 int=5
RParen 999:1000
Newline 1000:1001
Ident 1005:1006 text="b"
Dot 1006:1007
Ident 1007:1012 text="write"
LParen 1012:1013
Number 1013:1015 text="10" suf="" isf=0 int=10
RParen 1015:1016
Newline 1016:1017
Ident 1021:1027 text="expect"
LParen 1027:1028
Ident 1028:1029 text="b"
Dot 1029:1030
Ident 1030:1034 text="read"
LParen 1034:1035
RParen 1035:1036
RParen 1036:1037
Dot 1037:1038
Ident 1038:1042 text="toBe"
LParen 1042:1043
Number 1043:1045 text="10" suf="" isf=0 int=10
RParen 1045:1046
Newline 1046:1047
Ident 1051:1057 text="expect"
LParen 1057:1058
Ident 1058:1059 text="b"
Dot 1059:1060
Ident 1060:1065 text="value"
RParen 1065:1066
Dot 1066:1067
Ident 1067:1071 text="toBe"
LParen 1071:1072
Number 1072:1074 text="10" suf="" isf=0 int=10
RParen 1074:1075
Newline 1075:1076
RBrace 1076:1077
Newline 1077:1078
Newline 1078:1079
Ident 1079:1083 text="test"
Fn 1084:1086
Ident 1087:1107 text="generic_class_string"
LParen 1107:1108
RParen 1108:1109
LBrace 1110:1111
Newline 1111:1112
Var 1116:1119
Ident 1120:1121 text="s"
Assign 1122:1123
Ident 1124:1127 text="Box"
Lt 1127:1128
Ident 1128:1134 text="string"
Gt 1134:1135
LParen 1135:1136
Str 1136:1140 T("hi")
RParen 1140:1141
Newline 1141:1142
Ident 1146:1152 text="expect"
LParen 1152:1153
Ident 1153:1154 text="s"
Dot 1154:1155
Ident 1155:1159 text="read"
LParen 1159:1160
RParen 1160:1161
RParen 1161:1162
Dot 1162:1163
Ident 1163:1167 text="toBe"
LParen 1167:1168
Str 1168:1172 T("hi")
RParen 1172:1173
Newline 1173:1174
RBrace 1174:1175
Newline 1175:1176
Newline 1176:1177
Ident 1177:1181 text="test"
Fn 1182:1184
Ident 1185:1204 text="generic_class_float"
LParen 1204:1205
RParen 1205:1206
LBrace 1207:1208
Newline 1208:1209
Var 1213:1216
Ident 1217:1218 text="f"
Assign 1219:1220
Ident 1221:1224 text="Box"
Lt 1224:1225
Ident 1225:1230 text="float"
Gt 1230:1231
LParen 1231:1232
Number 1232:1235 text="2.5" suf="" isf=1 int=-
RParen 1235:1236
Newline 1236:1237
Ident 1241:1247 text="expect"
LParen 1247:1248
Ident 1248:1249 text="f"
Dot 1249:1250
Ident 1250:1255 text="value"
RParen 1255:1256
Dot 1256:1257
Ident 1257:1261 text="toBe"
LParen 1261:1262
Number 1262:1265 text="2.5" suf="" isf=1 int=-
RParen 1265:1266
Newline 1266:1267
RBrace 1267:1268
Newline 1268:1269
Newline 1269:1270
Ident 1270:1274 text="test"
Fn 1275:1277
Ident 1278:1303 text="generic_struct_two_params"
LParen 1303:1304
RParen 1304:1305
LBrace 1306:1307
Newline 1307:1308
Var 1312:1315
Ident 1316:1317 text="p"
Assign 1318:1319
Ident 1320:1324 text="Pair"
Lt 1324:1325
Ident 1325:1328 text="int"
Comma 1328:1329
Ident 1330:1336 text="string"
Gt 1336:1337
LParen 1337:1338
Number 1338:1339 text="7" suf="" isf=0 int=7
Comma 1339:1340
Str 1341:1348 T("seven")
RParen 1348:1349
Newline 1349:1350
Ident 1354:1360 text="expect"
LParen 1360:1361
Ident 1361:1362 text="p"
Dot 1362:1363
Ident 1363:1368 text="first"
RParen 1368:1369
Dot 1369:1370
Ident 1370:1374 text="toBe"
LParen 1374:1375
Number 1375:1376 text="7" suf="" isf=0 int=7
RParen 1376:1377
Newline 1377:1378
Ident 1382:1388 text="expect"
LParen 1388:1389
Ident 1389:1390 text="p"
Dot 1390:1391
Ident 1391:1397 text="second"
RParen 1397:1398
Dot 1398:1399
Ident 1399:1403 text="toBe"
LParen 1403:1404
Str 1404:1411 T("seven")
RParen 1411:1412
Newline 1412:1413
Ident 1417:1418 text="p"
Dot 1418:1419
Ident 1419:1424 text="first"
Assign 1425:1426
Number 1427:1428 text="8" suf="" isf=0 int=8
Newline 1428:1429
Ident 1433:1439 text="expect"
LParen 1439:1440
Ident 1440:1441 text="p"
Dot 1441:1442
Ident 1442:1447 text="first"
RParen 1447:1448
Dot 1448:1449
Ident 1449:1453 text="toBe"
LParen 1453:1454
Number 1454:1455 text="8" suf="" isf=0 int=8
RParen 1455:1456
Newline 1456:1457
RBrace 1457:1458
Newline 1458:1459
Newline 1459:1460
Ident 1460:1464 text="test"
Fn 1465:1467
Ident 1468:1492 text="generic_class_field_init"
LParen 1492:1493
RParen 1493:1494
LBrace 1495:1496
Newline 1496:1497
Var 1501:1504
Ident 1505:1506 text="c"
Assign 1507:1508
Ident 1509:1516 text="Counter"
Lt 1516:1517
Ident 1517:1520 text="int"
Gt 1520:1521
LParen 1521:1522
Number 1522:1524 text="42" suf="" isf=0 int=42
RParen 1524:1525
Newline 1525:1526
Ident 1530:1536 text="expect"
LParen 1536:1537
Ident 1537:1538 text="c"
Dot 1538:1539
Ident 1539:1543 text="bump"
LParen 1543:1544
RParen 1544:1545
RParen 1545:1546
Dot 1546:1547
Ident 1547:1551 text="toBe"
LParen 1551:1552
Number 1552:1553 text="1" suf="" isf=0 int=1
RParen 1553:1554
Newline 1554:1555
Ident 1559:1565 text="expect"
LParen 1565:1566
Ident 1566:1567 text="c"
Dot 1567:1568
Ident 1568:1572 text="bump"
LParen 1572:1573
RParen 1573:1574
RParen 1574:1575
Dot 1575:1576
Ident 1576:1580 text="toBe"
LParen 1580:1581
Number 1581:1582 text="2" suf="" isf=0 int=2
RParen 1582:1583
Newline 1583:1584
Ident 1588:1594 text="expect"
LParen 1594:1595
Ident 1595:1596 text="c"
Dot 1596:1597
Ident 1597:1601 text="item"
RParen 1601:1602
Dot 1602:1603
Ident 1603:1607 text="toBe"
LParen 1607:1608
Number 1608:1610 text="42" suf="" isf=0 int=42
RParen 1610:1611
Newline 1611:1612
RBrace 1612:1613
Newline 1613:1614
Newline 1614:1615
Ident 1615:1619 text="test"
Fn 1620:1622
Ident 1623:1647 text="generic_nested_list_type"
LParen 1647:1648
RParen 1648:1649
LBrace 1650:1651
Newline 1651:1652
Var 1656:1659
Ident 1660:1662 text="lb"
Assign 1663:1664
Ident 1665:1672 text="ListBox"
Lt 1672:1673
Ident 1673:1676 text="int"
Gt 1676:1677
LParen 1677:1678
LBracket 1678:1679
Number 1679:1680 text="3" suf="" isf=0 int=3
Comma 1680:1681
Number 1682:1683 text="1" suf="" isf=0 int=1
Comma 1683:1684
Number 1685:1686 text="2" suf="" isf=0 int=2
RBracket 1686:1687
Comma 1687:1688
Str 1689:1695 T("nums")
RParen 1695:1696
Newline 1696:1697
Ident 1701:1707 text="expect"
LParen 1707:1708
Ident 1708:1710 text="lb"
Dot 1710:1711
Ident 1711:1715 text="name"
RParen 1715:1716
Dot 1716:1717
Ident 1717:1721 text="toBe"
LParen 1721:1722
Str 1722:1728 T("nums")
RParen 1728:1729
Newline 1729:1730
Ident 1734:1740 text="expect"
LParen 1740:1741
Ident 1741:1743 text="lb"
Dot 1743:1744
Ident 1744:1753 text="firstItem"
LParen 1753:1754
Number 1754:1755 text="0" suf="" isf=0 int=0
RParen 1755:1756
RParen 1756:1757
Dot 1757:1758
Ident 1758:1762 text="toBe"
LParen 1762:1763
Number 1763:1764 text="3" suf="" isf=0 int=3
RParen 1764:1765
Newline 1765:1766
RBrace 1766:1767
Newline 1767:1768
Newline 1768:1769
Ident 1769:1773 text="test"
Fn 1774:1776
Ident 1777:1799 text="generic_swap_positions"
LParen 1799:1800
RParen 1800:1801
LBrace 1802:1803
Newline 1803:1804
Var 1808:1811
Ident 1812:1813 text="a"
Assign 1814:1815
Ident 1816:1820 text="Pair"
Lt 1820:1821
Ident 1821:1826 text="float"
Comma 1826:1827
Ident 1828:1834 text="string"
Gt 1834:1835
LParen 1835:1836
Number 1836:1839 text="1.5" suf="" isf=1 int=-
Comma 1839:1840
Str 1841:1844 T("x")
RParen 1844:1845
Newline 1845:1846
Var 1850:1853
Ident 1854:1855 text="b"
Assign 1856:1857
Ident 1858:1862 text="Pair"
Lt 1862:1863
Ident 1863:1869 text="string"
Comma 1869:1870
Ident 1871:1876 text="float"
Gt 1876:1877
LParen 1877:1878
Str 1878:1881 T("y")
Comma 1881:1882
Number 1883:1886 text="2.5" suf="" isf=1 int=-
RParen 1886:1887
Newline 1887:1888
Ident 1892:1898 text="expect"
LParen 1898:1899
Ident 1899:1900 text="a"
Dot 1900:1901
Ident 1901:1907 text="second"
RParen 1907:1908
Dot 1908:1909
Ident 1909:1913 text="toBe"
LParen 1913:1914
Str 1914:1917 T("x")
RParen 1917:1918
Newline 1918:1919
Ident 1923:1929 text="expect"
LParen 1929:1930
Ident 1930:1931 text="b"
Dot 1931:1932
Ident 1932:1938 text="second"
RParen 1938:1939
Dot 1939:1940
Ident 1940:1944 text="toBe"
LParen 1944:1945
Number 1945:1948 text="2.5" suf="" isf=1 int=-
RParen 1948:1949
Newline 1949:1950
RBrace 1950:1951
Newline 1951:1952
Newline 1952:1953
Ident 1953:1957 text="test"
Fn 1958:1960
Ident 1961:1992 text="generic_fn_taking_generic_class"
LParen 1992:1993
RParen 1993:1994
LBrace 1995:1996
Newline 1996:1997
Var 2001:2004
Ident 2005:2006 text="b"
Assign 2007:2008
Ident 2009:2012 text="Box"
Lt 2012:2013
Ident 2013:2016 text="int"
Gt 2016:2017
LParen 2017:2018
Number 2018:2019 text="5" suf="" isf=0 int=5
RParen 2019:2020
Newline 2020:2021
Ident 2025:2031 text="expect"
LParen 2031:2032
Ident 2032:2037 text="unbox"
Lt 2037:2038
Ident 2038:2041 text="int"
Gt 2041:2042
LParen 2042:2043
Ident 2043:2044 text="b"
RParen 2044:2045
RParen 2045:2046
Dot 2046:2047
Ident 2047:2051 text="toBe"
LParen 2051:2052
Number 2052:2053 text="5" suf="" isf=0 int=5
RParen 2053:2054
Newline 2054:2055
Ident 2059:2065 text="expect"
LParen 2065:2066
Ident 2066:2071 text="unbox"
Lt 2071:2072
Ident 2072:2078 text="string"
Gt 2078:2079
LParen 2079:2080
Ident 2080:2083 text="Box"
Lt 2083:2084
Ident 2084:2090 text="string"
Gt 2090:2091
LParen 2091:2092
Str 2092:2095 T("z")
RParen 2095:2096
RParen 2096:2097
RParen 2097:2098
Dot 2098:2099
Ident 2099:2103 text="toBe"
LParen 2103:2104
Str 2104:2107 T("z")
RParen 2107:2108
Newline 2108:2109
RBrace 2109:2110
Newline 2110:2111
Newline 2111:2112
Ident 2112:2116 text="test"
Fn 2117:2119
Ident 2120:2154 text="generic_fn_rebinding_generic_class"
LParen 2154:2155
RParen 2155:2156
LBrace 2157:2158
Newline 2158:2159
Var 2163:2166
Ident 2167:2168 text="b"
Assign 2169:2170
Ident 2171:2174 text="Box"
Lt 2174:2175
Ident 2175:2180 text="float"
Gt 2180:2181
LParen 2181:2182
Number 2182:2185 text="1.5" suf="" isf=1 int=-
RParen 2185:2186
Newline 2186:2187
Ident 2191:2197 text="expect"
LParen 2197:2198
Ident 2198:2202 text="swap"
Lt 2202:2203
Ident 2203:2208 text="float"
Gt 2208:2209
LParen 2209:2210
Ident 2210:2211 text="b"
Comma 2211:2212
Number 2213:2216 text="2.5" suf="" isf=1 int=-
RParen 2216:2217
RParen 2217:2218
Dot 2218:2219
Ident 2219:2223 text="toBe"
LParen 2223:2224
Number 2224:2227 text="1.5" suf="" isf=1 int=-
RParen 2227:2228
Newline 2228:2229
Ident 2233:2239 text="expect"
LParen 2239:2240
Ident 2240:2241 text="b"
Dot 2241:2242
Ident 2242:2246 text="read"
LParen 2246:2247
RParen 2247:2248
RParen 2248:2249
Dot 2249:2250
Ident 2250:2254 text="toBe"
LParen 2254:2255
Number 2255:2258 text="2.5" suf="" isf=1 int=-
RParen 2258:2259
Newline 2259:2260
RBrace 2260:2261
Newline 2261:2262
Newline 2262:2263
Ident 2263:2267 text="test"
Fn 2268:2270
Ident 2271:2296 text="nested_box_instantiations"
LParen 2296:2297
RParen 2297:2298
LBrace 2299:2300
Newline 2300:2301
Ident 2305:2311 text="expect"
LParen 2311:2312
Ident 2312:2324 text="nested_boxes"
LParen 2324:2325
RParen 2325:2326
RParen 2326:2327
Dot 2327:2328
Ident 2328:2332 text="toBe"
LParen 2332:2333
Number 2333:2334 text="7" suf="" isf=0 int=7
RParen 2334:2335
Newline 2335:2336
RBrace 2336:2337
Newline 2337:2338
Newline 2338:2339
Ident 2339:2343 text="test"
Fn 2344:2346
Ident 2347:2378 text="generic_fn_making_generic_class"
LParen 2378:2379
RParen 2379:2380
LBrace 2381:2382
Newline 2382:2383
Var 2387:2390
Ident 2391:2392 text="p"
Assign 2393:2394
Ident 2395:2404 text="make_pair"
Lt 2404:2405
Ident 2405:2408 text="int"
Comma 2408:2409
Ident 2410:2416 text="string"
Gt 2416:2417
LParen 2417:2418
Number 2418:2419 text="1" suf="" isf=0 int=1
Comma 2419:2420
Str 2421:2426 T("one")
RParen 2426:2427
Newline 2427:2428
Ident 2432:2438 text="expect"
LParen 2438:2439
Ident 2439:2440 text="p"
Dot 2440:2441
Ident 2441:2446 text="first"
RParen 2446:2447
Dot 2447:2448
Ident 2448:2452 text="toBe"
LParen 2452:2453
Number 2453:2454 text="1" suf="" isf=0 int=1
RParen 2454:2455
Newline 2455:2456
Ident 2460:2466 text="expect"
LParen 2466:2467
Ident 2467:2468 text="p"
Dot 2468:2469
Ident 2469:2475 text="second"
RParen 2475:2476
Dot 2476:2477
Ident 2477:2481 text="toBe"
LParen 2481:2482
Str 2482:2487 T("one")
RParen 2487:2488
Newline 2488:2489
RBrace 2489:2490
Newline 2490:2491
Newline 2491:2492
Ident 2492:2496 text="test"
Fn 2497:2499
Ident 2500:2524 text="nested_generic_type_args"
LParen 2524:2525
RParen 2525:2526
LBrace 2527:2528
Newline 2528:2529
Var 2533:2536
Ident 2537:2538 text="p"
Assign 2539:2540
Ident 2541:2545 text="Pair"
Lt 2545:2546
Ident 2546:2549 text="int"
Comma 2549:2550
Ident 2551:2554 text="Box"
Lt 2554:2555
Ident 2555:2561 text="string"
Shr 2561:2563
LParen 2563:2564
Number 2564:2565 text="3" suf="" isf=0 int=3
Comma 2565:2566
Ident 2567:2570 text="Box"
Lt 2570:2571
Ident 2571:2577 text="string"
Gt 2577:2578
LParen 2578:2579
Str 2579:2586 T("three")
RParen 2586:2587
RParen 2587:2588
Newline 2588:2589
Ident 2593:2599 text="expect"
LParen 2599:2600
Ident 2600:2601 text="p"
Dot 2601:2602
Ident 2602:2607 text="first"
RParen 2607:2608
Dot 2608:2609
Ident 2609:2613 text="toBe"
LParen 2613:2614
Number 2614:2615 text="3" suf="" isf=0 int=3
RParen 2615:2616
Newline 2616:2617
Ident 2621:2627 text="expect"
LParen 2627:2628
Ident 2628:2629 text="p"
Dot 2629:2630
Ident 2630:2636 text="second"
Dot 2636:2637
Ident 2637:2641 text="read"
LParen 2641:2642
RParen 2642:2643
RParen 2643:2644
Dot 2644:2645
Ident 2645:2649 text="toBe"
LParen 2649:2650
Str 2650:2657 T("three")
RParen 2657:2658
Newline 2658:2659
RBrace 2659:2660
Newline 2660:2661
Newline 2661:2662
Ident 2662:2666 text="test"
Fn 2667:2669
Ident 2670:2693 text="inferred_unbox_and_swap"
LParen 2693:2694
RParen 2694:2695
LBrace 2696:2697
Newline 2697:2698
Var 2702:2705
Ident 2706:2707 text="b"
Assign 2708:2709
Ident 2710:2713 text="Box"
Lt 2713:2714
Ident 2714:2717 text="int"
Gt 2717:2718
LParen 2718:2719
Number 2719:2720 text="5" suf="" isf=0 int=5
RParen 2720:2721
Newline 2721:2722
Ident 2726:2732 text="expect"
LParen 2732:2733
Ident 2733:2738 text="unbox"
LParen 2738:2739
Ident 2739:2740 text="b"
RParen 2740:2741
RParen 2741:2742
Dot 2742:2743
Ident 2743:2747 text="toBe"
LParen 2747:2748
Number 2748:2749 text="5" suf="" isf=0 int=5
RParen 2749:2750
Newline 2750:2751
Ident 2755:2761 text="expect"
LParen 2761:2762
Ident 2762:2767 text="unbox"
LParen 2767:2768
Ident 2768:2771 text="Box"
Lt 2771:2772
Ident 2772:2778 text="string"
Gt 2778:2779
LParen 2779:2780
Str 2780:2783 T("z")
RParen 2783:2784
RParen 2784:2785
RParen 2785:2786
Dot 2786:2787
Ident 2787:2791 text="toBe"
LParen 2791:2792
Str 2792:2795 T("z")
RParen 2795:2796
Newline 2796:2797
Ident 2801:2807 text="expect"
LParen 2807:2808
Ident 2808:2812 text="swap"
LParen 2812:2813
Ident 2813:2814 text="b"
Comma 2814:2815
Number 2816:2817 text="9" suf="" isf=0 int=9
RParen 2817:2818
RParen 2818:2819
Dot 2819:2820
Ident 2820:2824 text="toBe"
LParen 2824:2825
Number 2825:2826 text="5" suf="" isf=0 int=5
RParen 2826:2827
Newline 2827:2828
Ident 2832:2838 text="expect"
LParen 2838:2839
Ident 2839:2840 text="b"
Dot 2840:2841
Ident 2841:2845 text="read"
LParen 2845:2846
RParen 2846:2847
RParen 2847:2848
Dot 2848:2849
Ident 2849:2853 text="toBe"
LParen 2853:2854
Number 2854:2855 text="9" suf="" isf=0 int=9
RParen 2855:2856
Newline 2856:2857
RBrace 2857:2858
Eof 2858:2858
