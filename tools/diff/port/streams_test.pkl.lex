Ident 0:4 text="test"
Fn 5:7
Ident 8:35 text="stream_write_read_roundtrip"
LParen 35:36
RParen 36:37
LBrace 38:39
Newline 39:40
Let 44:47
Ident 48:52 text="path"
Assign 53:54
Str 55:92 T("tests/pickle/__stream_roundtrip.tmp")
Newline 92:93
Ident 97:103 text="delete"
LParen 103:104
Ident 104:108 text="path"
RParen 108:109
Newline 109:110
Let 114:117
Ident 118:119 text="s"
Assign 120:121
Ident 122:139 text="stream_open_write"
LParen 139:140
Ident 140:144 text="path"
RParen 144:145
Newline 145:146
If 150:152
LParen 153:154
Let 154:157
Ident 158:162 text="some"
LParen 162:163
Ident 163:164 text="w"
RParen 164:165
Assign 166:167
Ident 168:169 text="s"
RParen 169:170
LBrace 171:172
Newline 172:173
Ident 181:187 text="expect"
LParen 187:188
Ident 188:189 text="w"
Dot 189:190
Ident 190:195 text="write"
LParen 195:196
Ident 196:201 text="bytes"
LParen 201:202
Str 202:209 T("hello")
RParen 209:210
RParen 210:211
RParen 211:212
Dot 212:213
Ident 213:217 text="toBe"
LParen 217:218
Number 218:219 text="5" suf="" isf=0 int=5
RParen 219:220
Newline 220:221
Ident 229:235 text="expect"
LParen 235:236
Ident 236:237 text="w"
Dot 237:238
Ident 238:243 text="flush"
LParen 243:244
RParen 244:245
RParen 245:246
Dot 246:247
Ident 247:251 text="toBe"
LParen 251:252
True 252:256
RParen 256:257
Newline 257:258
Ident 266:272 text="expect"
LParen 272:273
Ident 273:274 text="w"
Dot 274:275
Ident 275:280 text="close"
LParen 280:281
RParen 281:282
RParen 282:283
Dot 283:284
Ident 284:288 text="toBe"
LParen 288:289
True 289:293
RParen 293:294
Newline 294:295
RBrace 299:300
Else 301:305
LBrace 306:307
Newline 307:308
Ident 316:322 text="expect"
LParen 322:323
Str 323:335 T("unexpected")
RParen 335:336
Dot 336:337
Ident 337:341 text="toBe"
LParen 341:342
Str 342:368 T("open write returned none")
RParen 368:369
Newline 369:370
Return 378:384
Newline 384:385
RBrace 389:390
Newline 390:391
Let 395:398
Ident 399:403 text="data"
Assign 404:405
Ident 406:415 text="read_file"
LParen 415:416
Ident 416:420 text="path"
RParen 420:421
Newline 421:422
If 426:428
LParen 429:430
Let 430:433
Ident 434:438 text="some"
LParen 438:439
Ident 439:442 text="txt"
RParen 442:443
Assign 444:445
Ident 446:450 text="data"
RParen 450:451
LBrace 452:453
Newline 453:454
Ident 462:468 text="expect"
LParen 468:469
Ident 469:472 text="txt"
RParen 472:473
Dot 473:474
Ident 474:478 text="toBe"
LParen 478:479
Str 479:486 T("hello")
RParen 486:487
Newline 487:488
RBrace 492:493
Else 494:498
LBrace 499:500
Newline 500:501
Ident 509:515 text="expect"
LParen 515:516
Str 516:528 T("unexpected")
RParen 528:529
Dot 529:530
Ident 530:534 text="toBe"
LParen 534:535
Str 535:571 T("roundtrip: read_file returned none")
RParen 571:572
Newline 572:573
Return 581:587
Newline 587:588
RBrace 592:593
Newline 593:594
Let 598:601
Ident 602:603 text="r"
Assign 604:605
Ident 606:622 text="stream_open_read"
LParen 622:623
Ident 623:627 text="path"
RParen 627:628
Newline 628:629
If 633:635
LParen 636:637
Let 637:640
Ident 641:645 text="some"
LParen 645:646
Ident 646:648 text="rd"
RParen 648:649
Assign 650:651
Ident 652:653 text="r"
RParen 653:654
LBrace 655:656
Newline 656:657
Var 665:668
Ident 669:672 text="out"
Assign 673:674
Str 675:677
Newline 677:678
While 686:691
LParen 692:693
True 693:697
RParen 697:698
LBrace 699:700
Newline 700:701
Let 713:716
Ident 717:722 text="chunk"
Assign 723:724
Ident 725:727 text="rd"
Dot 727:728
Ident 728:732 text="read"
LParen 732:733
Number 733:734 text="2" suf="" isf=0 int=2
RParen 734:735
Newline 735:736
If 748:750
LParen 751:752
Let 752:755
Ident 756:760 text="some"
LParen 760:761
Ident 761:762 text="c"
RParen 762:763
Assign 764:765
Ident 766:771 text="chunk"
RParen 771:772
LBrace 773:774
Newline 774:775
Ident 791:794 text="out"
PlusEq 795:797
Ident 798:801 text="str"
LParen 801:802
Ident 802:803 text="c"
RParen 803:804
Newline 804:805
RBrace 817:818
Else 819:823
LBrace 824:825
Newline 825:826
Break 842:847
Newline 847:848
RBrace 860:861
Newline 861:862
RBrace 870:871
Newline 871:872
Ident 880:886 text="expect"
LParen 886:887
Ident 887:890 text="out"
RParen 890:891
Dot 891:892
Ident 892:896 text="toBe"
LParen 896:897
Str 897:904 T("hello")
RParen 904:905
Newline 905:906
Ident 914:920 text="expect"
LParen 920:921
Ident 921:923 text="rd"
Dot 923:924
Ident 924:929 text="close"
LParen 929:930
RParen 930:931
RParen 931:932
Dot 932:933
Ident 933:937 text="toBe"
LParen 937:938
True 938:942
RParen 942:943
Newline 943:944
RBrace 948:949
Else 950:954
LBrace 955:956
Newline 956:957
Ident 965:971 text="expect"
LParen 971:972
Str 972:984 T("unexpected")
RParen 984:985
Dot 985:986
Ident 986:990 text="toBe"
LParen 990:991
Str 991:1016 T("open read returned none")
RParen 1016:1017
Newline 1017:1018
Return 1026:1032
Newline 1032:1033
RBrace 1037:1038
Newline 1038:1039
Ident 1043:1049 text="delete"
LParen 1049:1050
Ident 1050:1054 text="path"
RParen 1054:1055
Newline 1055:1056
RBrace 1056:1057
Newline 1057:1058
Newline 1058:1059
Ident 1059:1063 text="test"
Fn 1064:1066
Ident 1067:1101 text="stream_read_partial_chunks_and_eof"
LParen 1101:1102
RParen 1102:1103
LBrace 1104:1105
Newline 1105:1106
Let 1110:1113
Ident 1114:1118 text="path"
Assign 1119:1120
Str 1121:1152 T("tests/pickle/__stream_eof.tmp")
Newline 1152:1153
Ident 1157:1167 text="write_file"
LParen 1167:1168
Ident 1168:1172 text="path"
Comma 1172:1173
Str 1174:1182 T("abcdef")
RParen 1182:1183
Newline 1183:1184
Let 1188:1191
Ident 1192:1193 text="r"
Assign 1194:1195
Ident 1196:1212 text="stream_open_read"
LParen 1212:1213
Ident 1213:1217 text="path"
RParen 1217:1218
Newline 1218:1219
If 1223:1225
LParen 1226:1227
Let 1227:1230
Ident 1231:1235 text="some"
LParen 1235:1236
Ident 1236:1238 text="rd"
RParen 1238:1239
Assign 1240:1241
Ident 1242:1243 text="r"
RParen 1243:1244
LBrace 1245:1246
Newline 1246:1247
Newline 1317:1318
Let 1326:1329
Ident 1330:1335 text="first"
Assign 1336:1337
Ident 1338:1340 text="rd"
Dot 1340:1341
Ident 1341:1345 text="read"
LParen 1345:1346
Number 1346:1347 text="4" suf="" isf=0 int=4
RParen 1347:1348
Newline 1348:1349
If 1357:1359
LParen 1360:1361
Let 1361:1364
Ident 1365:1369 text="some"
LParen 1369:1370
Ident 1370:1371 text="c"
RParen 1371:1372
Assign 1373:1374
Ident 1375:1380 text="first"
RParen 1380:1381
LBrace 1382:1383
Newline 1383:1384
Ident 1396:1402 text="expect"
LParen 1402:1403
Ident 1403:1406 text="str"
LParen 1406:1407
Ident 1407:1408 text="c"
RParen 1408:1409
RParen 1409:1410
Dot 1410:1411
Ident 1411:1415 text="toBe"
LParen 1415:1416
Str 1416:1422 T("abcd")
RParen 1422:1423
Newline 1423:1424
RBrace 1432:1433
Else 1434:1438
LBrace 1439:1440
Newline 1440:1441
Ident 1453:1459 text="expect"
LParen 1459:1460
Str 1460:1472 T("unexpected")
RParen 1472:1473
Dot 1473:1474
Ident 1474:1478 text="toBe"
LParen 1478:1479
Str 1479:1500 T("first read was none")
RParen 1500:1501
Newline 1501:1502
RBrace 1510:1511
Newline 1511:1512
Let 1520:1523
Ident 1524:1530 text="second"
Assign 1531:1532
Ident 1533:1535 text="rd"
Dot 1535:1536
Ident 1536:1540 text="read"
LParen 1540:1541
Number 1541:1542 text="4" suf="" isf=0 int=4
RParen 1542:1543
Newline 1543:1544
If 1552:1554
LParen 1555:1556
Let 1556:1559
Ident 1560:1564 text="some"
LParen 1564:1565
Ident 1565:1566 text="c"
RParen 1566:1567
Assign 1568:1569
Ident 1570:1576 text="second"
RParen 1576:1577
LBrace 1578:1579
Newline 1579:1580
Ident 1592:1598 text="expect"
LParen 1598:1599
Ident 1599:1602 text="str"
LParen 1602:1603
Ident 1603:1604 text="c"
RParen 1604:1605
RParen 1605:1606
Dot 1606:1607
Ident 1607:1611 text="toBe"
LParen 1611:1612
Str 1612:1616 T("ef")
RParen 1616:1617
Newline 1617:1618
RBrace 1626:1627
Else 1628:1632
LBrace 1633:1634
Newline 1634:1635
Ident 1647:1653 text="expect"
LParen 1653:1654
Str 1654:1666 T("unexpected")
RParen 1666:1667
Dot 1667:1668
Ident 1668:1672 text="toBe"
LParen 1672:1673
Str 1673:1702 T("final partial read was none")
RParen 1702:1703
Newline 1703:1704
RBrace 1712:1713
Newline 1713:1714
Newline 1758:1759
Let 1767:1770
Ident 1771:1776 text="third"
Assign 1777:1778
Ident 1779:1781 text="rd"
Dot 1781:1782
Ident 1782:1786 text="read"
LParen 1786:1787
Number 1787:1788 text="2" suf="" isf=0 int=2
RParen 1788:1789
Newline 1789:1790
If 1798:1800
LParen 1801:1802
Let 1802:1805
Ident 1806:1810 text="some"
LParen 1810:1811
Ident 1811:1813 text="_c"
RParen 1813:1814
Assign 1815:1816
Ident 1817:1822 text="third"
RParen 1822:1823
LBrace 1824:1825
Newline 1825:1826
Ident 1838:1844 text="expect"
LParen 1844:1845
Str 1845:1857 T("unexpected")
RParen 1857:1858
Dot 1858:1859
Ident 1859:1863 text="toBe"
LParen 1863:1864
Str 1864:1893 T("read past EOF returned some")
RParen 1893:1894
Newline 1894:1895
RBrace 1903:1904
Else 1905:1909
LBrace 1910:1911
Newline 1911:1912
Ident 1924:1930 text="expect"
LParen 1930:1931
True 1931:1935
RParen 1935:1936
Dot 1936:1937
Ident 1937:1941 text="toBe"
LParen 1941:1942
True 1942:1946
RParen 1946:1947
Newline 1947:1948
RBrace 1956:1957
Newline 1957:1958
Ident 1966:1968 text="rd"
Dot 1968:1969
Ident 1969:1974 text="close"
LParen 1974:1975
RParen 1975:1976
Newline 1976:1977
RBrace 1981:1982
Else 1983:1987
LBrace 1988:1989
Newline 1989:1990
Ident 1998:2004 text="expect"
LParen 2004:2005
Str 2005:2017 T("unexpected")
RParen 2017:2018
Dot 2018:2019
Ident 2019:2023 text="toBe"
LParen 2023:2024
Str 2024:2049 T("open read returned none")
RParen 2049:2050
Newline 2050:2051
RBrace 2055:2056
Newline 2056:2057
Ident 2061:2067 text="delete"
LParen 2067:2068
Ident 2068:2072 text="path"
RParen 2072:2073
Newline 2073:2074
RBrace 2074:2075
Newline 2075:2076
Newline 2076:2077
Ident 2077:2081 text="test"
Fn 2082:2084
Ident 2085:2115 text="stream_append_extends_existing"
LParen 2115:2116
RParen 2116:2117
LBrace 2118:2119
Newline 2119:2120
Let 2124:2127
Ident 2128:2132 text="path"
Assign 2133:2134
Str 2135:2169 T("tests/pickle/__stream_append.tmp")
Newline 2169:2170
Ident 2174:2184 text="write_file"
LParen 2184:2185
Ident 2185:2189 text="path"
Comma 2189:2190
Str 2191:2195 T("ab")
RParen 2195:2196
Newline 2196:2197
Let 2201:2204
Ident 2205:2206 text="s"
Assign 2207:2208
Ident 2209:2227 text="stream_open_append"
LParen 2227:2228
Ident 2228:2232 text="path"
RParen 2232:2233
Newline 2233:2234
If 2238:2240
LParen 2241:2242
Let 2242:2245
Ident 2246:2250 text="some"
LParen 2250:2251
Ident 2251:2252 text="w"
RParen 2252:2253
Assign 2254:2255
Ident 2256:2257 text="s"
RParen 2257:2258
LBrace 2259:2260
Newline 2260:2261
Ident 2269:2275 text="expect"
LParen 2275:2276
Ident 2276:2277 text="w"
Dot 2277:2278
Ident 2278:2283 text="write"
LParen 2283:2284
Ident 2284:2289 text="bytes"
LParen 2289:2290
Str 2290:2294 T("cd")
RParen 2294:2295
RParen 2295:2296
RParen 2296:2297
Dot 2297:2298
Ident 2298:2302 text="toBe"
LParen 2302:2303
Number 2303:2304 text="2" suf="" isf=0 int=2
RParen 2304:2305
Newline 2305:2306
Ident 2314:2315 text="w"
Dot 2315:2316
Ident 2316:2321 text="close"
LParen 2321:2322
RParen 2322:2323
Newline 2323:2324
RBrace 2328:2329
Else 2330:2334
LBrace 2335:2336
Newline 2336:2337
Ident 2345:2351 text="expect"
LParen 2351:2352
Str 2352:2364 T("unexpected")
RParen 2364:2365
Dot 2365:2366
Ident 2366:2370 text="toBe"
LParen 2370:2371
Str 2371:2398 T("open append returned none")
RParen 2398:2399
Newline 2399:2400
Return 2408:2414
Newline 2414:2415
RBrace 2419:2420
Newline 2420:2421
Let 2425:2428
Ident 2429:2433 text="data"
Assign 2434:2435
Ident 2436:2445 text="read_file"
LParen 2445:2446
Ident 2446:2450 text="path"
RParen 2450:2451
Newline 2451:2452
If 2456:2458
LParen 2459:2460
Let 2460:2463
Ident 2464:2468 text="some"
LParen 2468:2469
Ident 2469:2472 text="txt"
RParen 2472:2473
Assign 2474:2475
Ident 2476:2480 text="data"
RParen 2480:2481
LBrace 2482:2483
Newline 2483:2484
Ident 2492:2498 text="expect"
LParen 2498:2499
Ident 2499:2502 text="txt"
RParen 2502:2503
Dot 2503:2504
Ident 2504:2508 text="toBe"
LParen 2508:2509
Str 2509:2515 T("abcd")
RParen 2515:2516
Newline 2516:2517
RBrace 2521:2522
Else 2523:2527
LBrace 2528:2529
Newline 2529:2530
Ident 2538:2544 text="expect"
LParen 2544:2545
Str 2545:2557 T("unexpected")
RParen 2557:2558
Dot 2558:2559
Ident 2559:2563 text="toBe"
LParen 2563:2564
Str 2564:2597 T("append: read_file returned none")
RParen 2597:2598
Newline 2598:2599
RBrace 2603:2604
Newline 2604:2605
Ident 2609:2615 text="delete"
LParen 2615:2616
Ident 2616:2620 text="path"
RParen 2620:2621
Newline 2621:2622
RBrace 2622:2623
Newline 2623:2624
Newline 2624:2625
Ident 2625:2629 text="test"
Fn 2630:2632
Ident 2633:2663 text="stream_write_open_creates_file"
LParen 2663:2664
RParen 2664:2665
LBrace 2666:2667
Newline 2667:2668
Let 2672:2675
Ident 2676:2680 text="path"
Assign 2681:2682
Str 2683:2717 T("tests/pickle/__stream_create.tmp")
Newline 2717:2718
Ident 2722:2728 text="delete"
LParen 2728:2729
Ident 2729:2733 text="path"
RParen 2733:2734
Newline 2734:2735
Let 2739:2742
Ident 2743:2744 text="s"
Assign 2745:2746
Ident 2747:2764 text="stream_open_write"
LParen 2764:2765
Ident 2765:2769 text="path"
RParen 2769:2770
Newline 2770:2771
If 2775:2777
LParen 2778:2779
Let 2779:2782
Ident 2783:2787 text="some"
LParen 2787:2788
Ident 2788:2789 text="w"
RParen 2789:2790
Assign 2791:2792
Ident 2793:2794 text="s"
RParen 2794:2795
LBrace 2796:2797
Newline 2797:2798
Ident 2806:2807 text="w"
Dot 2807:2808
Ident 2808:2813 text="write"
LParen 2813:2814
Ident 2814:2819 text="bytes"
LParen 2819:2820
Str 2820:2823 T("x")
RParen 2823:2824
RParen 2824:2825
Newline 2825:2826
Ident 2834:2835 text="w"
Dot 2835:2836
Ident 2836:2841 text="close"
LParen 2841:2842
RParen 2842:2843
Newline 2843:2844
RBrace 2848:2849
Else 2850:2854
LBrace 2855:2856
Newline 2856:2857
Ident 2865:2871 text="expect"
LParen 2871:2872
Str 2872:2884 T("unexpected")
RParen 2884:2885
Dot 2885:2886
Ident 2886:2890 text="toBe"
LParen 2890:2891
Str 2891:2917 T("open write returned none")
RParen 2917:2918
Newline 2918:2919
RBrace 2923:2924
Newline 2924:2925
Ident 2929:2935 text="expect"
LParen 2935:2936
Ident 2936:2947 text="file_exists"
LParen 2947:2948
Ident 2948:2952 text="path"
RParen 2952:2953
RParen 2953:2954
Dot 2954:2955
Ident 2955:2959 text="toBe"
LParen 2959:2960
True 2960:2964
RParen 2964:2965
Newline 2965:2966
Ident 2970:2976 text="delete"
LParen 2976:2977
Ident 2977:2981 text="path"
RParen 2981:2982
Newline 2982:2983
RBrace 2983:2984
Newline 2984:2985
Newline 2985:2986
Ident 2986:2990 text="test"
Fn 2991:2993
Ident 2994:3021 text="stream_missing_path_is_none"
LParen 3021:3022
RParen 3022:3023
LBrace 3024:3025
Newline 3025:3026
Let 3030:3033
Ident 3034:3035 text="r"
Assign 3036:3037
Ident 3038:3054 text="stream_open_read"
LParen 3054:3055
Str 3055:3095 T("tests/pickle/__stream_never_exists.tmp")
RParen 3095:3096
Newline 3096:3097
If 3101:3103
LParen 3104:3105
Let 3105:3108
Ident 3109:3113 text="some"
LParen 3113:3114
Ident 3114:3116 text="_s"
RParen 3116:3117
Assign 3118:3119
Ident 3120:3121 text="r"
RParen 3121:3122
LBrace 3123:3124
Newline 3124:3125
Ident 3133:3139 text="expect"
LParen 3139:3140
Str 3140:3152 T("unexpected")
RParen 3152:3153
Dot 3153:3154
Ident 3154:3158 text="toBe"
LParen 3158:3159
Str 3159:3185 T("missing file opened read")
RParen 3185:3186
Newline 3186:3187
RBrace 3191:3192
Else 3193:3197
LBrace 3198:3199
Newline 3199:3200
Ident 3208:3214 text="expect"
LParen 3214:3215
True 3215:3219
RParen 3219:3220
Dot 3220:3221
Ident 3221:3225 text="toBe"
LParen 3225:3226
True 3226:3230
RParen 3230:3231
Newline 3231:3232
RBrace 3236:3237
Newline 3237:3238
RBrace 3238:3239
Newline 3239:3240
Newline 3240:3241
Ident 3241:3245 text="test"
Fn 3246:3248
Ident 3249:3275 text="stream_close_is_idempotent"
LParen 3275:3276
RParen 3276:3277
LBrace 3278:3279
Newline 3279:3280
Let 3284:3287
Ident 3288:3292 text="path"
Assign 3293:3294
Str 3295:3328 T("tests/pickle/__stream_close.tmp")
Newline 3328:3329
Let 3333:3336
Ident 3337:3338 text="s"
Assign 3339:3340
Ident 3341:3358 text="stream_open_write"
LParen 3358:3359
Ident 3359:3363 text="path"
RParen 3363:3364
Newline 3364:3365
If 3369:3371
LParen 3372:3373
Let 3373:3376
Ident 3377:3381 text="some"
LParen 3381:3382
Ident 3382:3383 text="w"
RParen 3383:3384
Assign 3385:3386
Ident 3387:3388 text="s"
RParen 3388:3389
LBrace 3390:3391
Newline 3391:3392
Ident 3400:3406 text="expect"
LParen 3406:3407
Ident 3407:3408 text="w"
Dot 3408:3409
Ident 3409:3414 text="close"
LParen 3414:3415
RParen 3415:3416
RParen 3416:3417
Dot 3417:3418
Ident 3418:3422 text="toBe"
LParen 3422:3423
True 3423:3427
RParen 3427:3428
Newline 3428:3429
Ident 3437:3443 text="expect"
LParen 3443:3444
Ident 3444:3445 text="w"
Dot 3445:3446
Ident 3446:3451 text="close"
LParen 3451:3452
RParen 3452:3453
RParen 3453:3454
Dot 3454:3455
Ident 3455:3459 text="toBe"
LParen 3459:3460
False 3460:3465
RParen 3465:3466
Newline 3466:3467
Ident 3475:3481 text="expect"
LParen 3481:3482
Ident 3482:3483 text="w"
Dot 3483:3484
Ident 3484:3489 text="write"
LParen 3489:3490
Ident 3490:3495 text="bytes"
LParen 3495:3496
Str 3496:3499 T("x")
RParen 3499:3500
RParen 3500:3501
RParen 3501:3502
Dot 3502:3503
Ident 3503:3507 text="toBe"
LParen 3507:3508
Number 3508:3509 text="0" suf="" isf=0 int=0
RParen 3509:3510
Newline 3510:3511
Ident 3519:3525 text="expect"
LParen 3525:3526
Ident 3526:3527 text="w"
Dot 3527:3528
Ident 3528:3533 text="flush"
LParen 3533:3534
RParen 3534:3535
RParen 3535:3536
Dot 3536:3537
Ident 3537:3541 text="toBe"
LParen 3541:3542
False 3542:3547
RParen 3547:3548
Newline 3548:3549
RBrace 3553:3554
Else 3555:3559
LBrace 3560:3561
Newline 3561:3562
Ident 3570:3576 text="expect"
LParen 3576:3577
Str 3577:3589 T("unexpected")
RParen 3589:3590
Dot 3590:3591
Ident 3591:3595 text="toBe"
LParen 3595:3596
Str 3596:3622 T("open write returned none")
RParen 3622:3623
Newline 3623:3624
RBrace 3628:3629
Newline 3629:3630
Ident 3634:3640 text="delete"
LParen 3640:3641
Ident 3641:3645 text="path"
RParen 3645:3646
Newline 3646:3647
RBrace 3647:3648
Newline 3648:3649
Newline 3649:3650
Ident 3650:3654 text="test"
Fn 3655:3657
Ident 3658:3703 text="stream_flush_makes_data_visible_without_close"
LParen 3703:3704
RParen 3704:3705
LBrace 3706:3707
Newline 3707:3708
Let 3712:3715
Ident 3716:3720 text="path"
Assign 3721:3722
Str 3723:3756 T("tests/pickle/__stream_flush.tmp")
Newline 3756:3757
Let 3761:3764
Ident 3765:3766 text="s"
Assign 3767:3768
Ident 3769:3786 text="stream_open_write"
LParen 3786:3787
Ident 3787:3791 text="path"
RParen 3791:3792
Newline 3792:3793
If 3797:3799
LParen 3800:3801
Let 3801:3804
Ident 3805:3809 text="some"
LParen 3809:3810
Ident 3810:3811 text="w"
RParen 3811:3812
Assign 3813:3814
Ident 3815:3816 text="s"
RParen 3816:3817
LBrace 3818:3819
Newline 3819:3820
Ident 3828:3829 text="w"
Dot 3829:3830
Ident 3830:3835 text="write"
LParen 3835:3836
Ident 3836:3841 text="bytes"
LParen 3841:3842
Str 3842:3848 T("data")
RParen 3848:3849
RParen 3849:3850
Newline 3850:3851
Ident 3859:3865 text="expect"
LParen 3865:3866
Ident 3866:3867 text="w"
Dot 3867:3868
Ident 3868:3873 text="flush"
LParen 3873:3874
RParen 3874:3875
RParen 3875:3876
Dot 3876:3877
Ident 3877:3881 text="toBe"
LParen 3881:3882
True 3882:3886
RParen 3886:3887
Newline 3887:3888
RBrace 3892:3893
Else 3894:3898
LBrace 3899:3900
Newline 3900:3901
Ident 3909:3915 text="expect"
LParen 3915:3916
Str 3916:3928 T("unexpected")
RParen 3928:3929
Dot 3929:3930
Ident 3930:3934 text="toBe"
LParen 3934:3935
Str 3935:3961 T("open write returned none")
RParen 3961:3962
Newline 3962:3963
Return 3971:3977
Newline 3977:3978
RBrace 3982:3983
Newline 3983:3984
Let 3988:3991
Ident 3992:3996 text="data"
Assign 3997:3998
Ident 3999:4008 text="read_file"
LParen 4008:4009
Ident 4009:4013 text="path"
RParen 4013:4014
Newline 4014:4015
If 4019:4021
LParen 4022:4023
Let 4023:4026
Ident 4027:4031 text="some"
LParen 4031:4032
Ident 4032:4035 text="txt"
RParen 4035:4036
Assign 4037:4038
Ident 4039:4043 text="data"
RParen 4043:4044
LBrace 4045:4046
Newline 4046:4047
Ident 4055:4061 text="expect"
LParen 4061:4062
Ident 4062:4065 text="txt"
RParen 4065:4066
Dot 4066:4067
Ident 4067:4071 text="toBe"
LParen 4071:4072
Str 4072:4078 T("data")
RParen 4078:4079
Newline 4079:4080
RBrace 4084:4085
Else 4086:4090
LBrace 4091:4092
Newline 4092:4093
Ident 4101:4107 text="expect"
LParen 4107:4108
Str 4108:4120 T("unexpected")
RParen 4120:4121
Dot 4121:4122
Ident 4122:4126 text="toBe"
LParen 4126:4127
Str 4127:4159 T("flush: read_file returned none")
RParen 4159:4160
Newline 4160:4161
RBrace 4165:4166
Newline 4166:4167
Ident 4171:4177 text="delete"
LParen 4177:4178
Ident 4178:4182 text="path"
RParen 4182:4183
Newline 4183:4184
RBrace 4184:4185
Newline 4185:4186
Newline 4186:4187
Ident 4187:4191 text="test"
Fn 4192:4194
Ident 4195:4228 text="stream_console_streams_are_usable"
LParen 4228:4229
RParen 4229:4230
LBrace 4231:4232
Newline 4232:4233
Let 4237:4240
Ident 4241:4244 text="out"
Assign 4245:4246
Ident 4247:4260 text="stdout_stream"
LParen 4260:4261
RParen 4261:4262
Newline 4262:4263
Let 4267:4270
Ident 4271:4274 text="err"
Assign 4275:4276
Ident 4277:4290 text="stderr_stream"
LParen 4290:4291
RParen 4291:4292
Newline 4292:4293
Newline 4366:4367
Newline 4417:4418
Ident 4422:4428 text="expect"
LParen 4428:4429
Ident 4429:4432 text="out"
Dot 4432:4433
Ident 4433:4438 text="write"
LParen 4438:4439
Ident 4439:4444 text="bytes"
LParen 4444:4445
Str 4445:4447
RParen 4447:4448
RParen 4448:4449
RParen 4449:4450
Dot 4450:4451
Ident 4451:4455 text="toBe"
LParen 4455:4456
Number 4456:4457 text="0" suf="" isf=0 int=0
RParen 4457:4458
Newline 4458:4459
Ident 4463:4469 text="expect"
LParen 4469:4470
Ident 4470:4473 text="err"
Dot 4473:4474
Ident 4474:4479 text="write"
LParen 4479:4480
Ident 4480:4485 text="bytes"
LParen 4485:4486
Str 4486:4488
RParen 4488:4489
RParen 4489:4490
RParen 4490:4491
Dot 4491:4492
Ident 4492:4496 text="toBe"
LParen 4496:4497
Number 4497:4498 text="0" suf="" isf=0 int=0
RParen 4498:4499
Newline 4499:4500
Ident 4504:4510 text="expect"
LParen 4510:4511
Ident 4511:4514 text="out"
Dot 4514:4515
Ident 4515:4520 text="flush"
LParen 4520:4521
RParen 4521:4522
RParen 4522:4523
Dot 4523:4524
Ident 4524:4528 text="toBe"
LParen 4528:4529
True 4529:4533
RParen 4533:4534
Newline 4534:4535
Ident 4539:4545 text="expect"
LParen 4545:4546
Ident 4546:4549 text="err"
Dot 4549:4550
Ident 4550:4555 text="flush"
LParen 4555:4556
RParen 4556:4557
RParen 4557:4558
Dot 4558:4559
Ident 4559:4563 text="toBe"
LParen 4563:4564
True 4564:4568
RParen 4568:4569
Newline 4569:4570
Newline 4642:4643
RBrace 4643:4644
Eof 4644:4644
