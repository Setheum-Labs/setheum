/// This is autogenerated by script. Please don't directly change it.
/// Under the utils/inverse-function folder,
/// Run `cargo run > ../../serml/staking/src/total_stake_limit_ratio.rs`

use sp_runtime::{Perbill, Permill};

pub fn total_stake_limit_ratio(effective_stake_ratio: Permill) -> (u32, Perbill) {
    match effective_stake_ratio.deconstruct() {
        0 ..= 999 => (659, Perbill::from_parts(350000000)),
        1000 ..= 1999 => (219, Perbill::from_parts(349999999)),
        2000 ..= 2999 => (131, Perbill::from_parts(349999999)),
        3000 ..= 3999 => (93, Perbill::from_parts(635714285)),
        4000 ..= 4999 => (72, Perbill::from_parts(683333333)),
        5000 ..= 5999 => (59, Perbill::from_parts(350000000)),
        6000 ..= 6999 => (50, Perbill::from_parts(119230769)),
        7000 ..= 7999 => (43, Perbill::from_parts(350000000)),
        8000 ..= 8999 => (38, Perbill::from_parts(173529411)),
        9000 ..= 9999 => (34, Perbill::from_parts(86842105)),
        10000 ..= 10999 => (30, Perbill::from_parts(778571428)),
        11000 ..= 11999 => (28, Perbill::from_parts(45652173)),
        12000 ..= 12999 => (25, Perbill::from_parts(750000000)),
        13000 ..= 13999 => (23, Perbill::from_parts(794444444)),
        14000 ..= 14999 => (22, Perbill::from_parts(108620689)),
        15000 ..= 15999 => (20, Perbill::from_parts(640322580)),
        16000 ..= 16999 => (19, Perbill::from_parts(350000000)),
        17000 ..= 17999 => (18, Perbill::from_parts(207142857)),
        18000 ..= 18999 => (17, Perbill::from_parts(187837837)),
        19000 ..= 19999 => (16, Perbill::from_parts(273076923)),
        20000 ..= 20999 => (15, Perbill::from_parts(447560975)),
        21000 ..= 21999 => (14, Perbill::from_parts(698837209)),
        22000 ..= 22999 => (14, Perbill::from_parts(16666666)),
        23000 ..= 23999 => (13, Perbill::from_parts(392553191)),
        24000 ..= 24999 => (12, Perbill::from_parts(819387755)),
        25000 ..= 25999 => (12, Perbill::from_parts(291176470)),
        26000 ..= 26999 => (11, Perbill::from_parts(802830188)),
        27000 ..= 27999 => (11, Perbill::from_parts(349999999)),
        28000 ..= 28999 => (10, Perbill::from_parts(928947368)),
        29000 ..= 29999 => (10, Perbill::from_parts(536440677)),
        30000 ..= 30999 => (10, Perbill::from_parts(169672131)),
        31000 ..= 31999 => (9, Perbill::from_parts(826190476)),
        32000 ..= 32999 => (9, Perbill::from_parts(503846153)),
        33000 ..= 33999 => (9, Perbill::from_parts(200746268)),
        34000 ..= 34999 => (8, Perbill::from_parts(915217391)),
        35000 ..= 35999 => (8, Perbill::from_parts(645774647)),
        36000 ..= 36999 => (8, Perbill::from_parts(391095890)),
        37000 ..= 37999 => (8, Perbill::from_parts(150000000)),
        38000 ..= 38999 => (7, Perbill::from_parts(921428571)),
        39000 ..= 39999 => (7, Perbill::from_parts(704430379)),
        40000 ..= 40999 => (7, Perbill::from_parts(498148148)),
        41000 ..= 41999 => (7, Perbill::from_parts(301807228)),
        42000 ..= 42999 => (7, Perbill::from_parts(114705882)),
        43000 ..= 43999 => (6, Perbill::from_parts(936206896)),
        44000 ..= 44999 => (6, Perbill::from_parts(765730337)),
        45000 ..= 45999 => (6, Perbill::from_parts(602747252)),
        46000 ..= 46999 => (6, Perbill::from_parts(446774193)),
        47000 ..= 47999 => (6, Perbill::from_parts(297368421)),
        48000 ..= 48999 => (6, Perbill::from_parts(154123711)),
        49000 ..= 49999 => (6, Perbill::from_parts(16666666)),
        50000 ..= 50999 => (5, Perbill::from_parts(884653465)),
        51000 ..= 51999 => (5, Perbill::from_parts(757766990)),
        52000 ..= 52999 => (5, Perbill::from_parts(635714285)),
        53000 ..= 53999 => (5, Perbill::from_parts(518224299)),
        54000 ..= 54999 => (5, Perbill::from_parts(405045871)),
        55000 ..= 55999 => (5, Perbill::from_parts(295945945)),
        56000 ..= 56999 => (5, Perbill::from_parts(190707964)),
        57000 ..= 57999 => (5, Perbill::from_parts(89130434)),
        58000 ..= 58999 => (4, Perbill::from_parts(991025641)),
        59000 ..= 59999 => (4, Perbill::from_parts(896218487)),
        60000 ..= 60999 => (4, Perbill::from_parts(804545454)),
        61000 ..= 61999 => (4, Perbill::from_parts(715853658)),
        62000 ..= 62999 => (4, Perbill::from_parts(629999999)),
        63000 ..= 63999 => (4, Perbill::from_parts(546850393)),
        64000 ..= 64999 => (4, Perbill::from_parts(466279069)),
        65000 ..= 65999 => (4, Perbill::from_parts(388167938)),
        66000 ..= 66999 => (4, Perbill::from_parts(312406015)),
        67000 ..= 67999 => (4, Perbill::from_parts(238888888)),
        68000 ..= 68999 => (4, Perbill::from_parts(167518248)),
        69000 ..= 69999 => (4, Perbill::from_parts(98201438)),
        70000 ..= 70999 => (4, Perbill::from_parts(30851063)),
        71000 ..= 71999 => (3, Perbill::from_parts(965384615)),
        72000 ..= 72999 => (3, Perbill::from_parts(901724137)),
        73000 ..= 73999 => (3, Perbill::from_parts(839795918)),
        74000 ..= 74999 => (3, Perbill::from_parts(779530201)),
        75000 ..= 75999 => (3, Perbill::from_parts(720860927)),
        76000 ..= 76999 => (3, Perbill::from_parts(663725490)),
        77000 ..= 77999 => (3, Perbill::from_parts(608064516)),
        78000 ..= 78999 => (3, Perbill::from_parts(553821656)),
        79000 ..= 79999 => (3, Perbill::from_parts(500943396)),
        80000 ..= 80999 => (3, Perbill::from_parts(449378881)),
        81000 ..= 81999 => (3, Perbill::from_parts(399079754)),
        82000 ..= 82999 => (3, Perbill::from_parts(350000000)),
        83000 ..= 83999 => (3, Perbill::from_parts(302095808)),
        84000 ..= 84999 => (3, Perbill::from_parts(255325443)),
        85000 ..= 85999 => (3, Perbill::from_parts(209649122)),
        86000 ..= 86999 => (3, Perbill::from_parts(165028901)),
        87000 ..= 87999 => (3, Perbill::from_parts(121428571)),
        88000 ..= 88999 => (3, Perbill::from_parts(78813559)),
        89000 ..= 89999 => (3, Perbill::from_parts(37150837)),
        90000 ..= 90999 => (2, Perbill::from_parts(996408839)),
        91000 ..= 91999 => (2, Perbill::from_parts(956557377)),
        92000 ..= 92999 => (2, Perbill::from_parts(917567567)),
        93000 ..= 93999 => (2, Perbill::from_parts(879411764)),
        94000 ..= 94999 => (2, Perbill::from_parts(842063492)),
        95000 ..= 95999 => (2, Perbill::from_parts(805497382)),
        96000 ..= 96999 => (2, Perbill::from_parts(769689119)),
        97000 ..= 97999 => (2, Perbill::from_parts(734615384)),
        98000 ..= 98999 => (2, Perbill::from_parts(700253807)),
        99000 ..= 99999 => (2, Perbill::from_parts(666582914)),
        100000 ..= 100999 => (2, Perbill::from_parts(633582089)),
        101000 ..= 101999 => (2, Perbill::from_parts(601231527)),
        102000 ..= 102999 => (2, Perbill::from_parts(569512195)),
        103000 ..= 103999 => (2, Perbill::from_parts(538405797)),
        104000 ..= 104999 => (2, Perbill::from_parts(507894736)),
        105000 ..= 105999 => (2, Perbill::from_parts(477962085)),
        106000 ..= 106999 => (2, Perbill::from_parts(448591549)),
        107000 ..= 107999 => (2, Perbill::from_parts(419767441)),
        108000 ..= 108999 => (2, Perbill::from_parts(391474654)),
        109000 ..= 109999 => (2, Perbill::from_parts(363698630)),
        110000 ..= 110999 => (2, Perbill::from_parts(336425339)),
        111000 ..= 111999 => (2, Perbill::from_parts(309641255)),
        112000 ..= 112999 => (2, Perbill::from_parts(283333333)),
        113000 ..= 113999 => (2, Perbill::from_parts(257488986)),
        114000 ..= 114999 => (2, Perbill::from_parts(232096069)),
        115000 ..= 115999 => (2, Perbill::from_parts(207142857)),
        116000 ..= 116999 => (2, Perbill::from_parts(182618025)),
        117000 ..= 117999 => (2, Perbill::from_parts(158510638)),
        118000 ..= 118999 => (2, Perbill::from_parts(134810126)),
        119000 ..= 119999 => (2, Perbill::from_parts(111506276)),
        120000 ..= 120999 => (2, Perbill::from_parts(88589211)),
        121000 ..= 121999 => (2, Perbill::from_parts(66049382)),
        122000 ..= 122999 => (2, Perbill::from_parts(43877551)),
        123000 ..= 123999 => (2, Perbill::from_parts(22064777)),
        124000 ..= 124999 => (2, Perbill::from_parts(602409)),
        125000 ..= 125999 => (1, Perbill::from_parts(979482071)),
        126000 ..= 126999 => (1, Perbill::from_parts(958695652)),
        127000 ..= 127999 => (1, Perbill::from_parts(938235294)),
        128000 ..= 128999 => (1, Perbill::from_parts(918093385)),
        129000 ..= 129999 => (1, Perbill::from_parts(898262548)),
        130000 ..= 130999 => (1, Perbill::from_parts(878735632)),
        131000 ..= 131999 => (1, Perbill::from_parts(859505703)),
        132000 ..= 132999 => (1, Perbill::from_parts(840566037)),
        133000 ..= 133999 => (1, Perbill::from_parts(821910112)),
        134000 ..= 134999 => (1, Perbill::from_parts(803531598)),
        135000 ..= 135999 => (1, Perbill::from_parts(785424354)),
        136000 ..= 136999 => (1, Perbill::from_parts(767582417)),
        137000 ..= 137999 => (1, Perbill::from_parts(750000000)),
        138000 ..= 138999 => (1, Perbill::from_parts(732671480)),
        139000 ..= 139999 => (1, Perbill::from_parts(715591397)),
        140000 ..= 140999 => (1, Perbill::from_parts(698754448)),
        141000 ..= 141999 => (1, Perbill::from_parts(682155477)),
        142000 ..= 142999 => (1, Perbill::from_parts(665789473)),
        143000 ..= 143999 => (1, Perbill::from_parts(649651567)),
        144000 ..= 144999 => (1, Perbill::from_parts(633737024)),
        145000 ..= 145999 => (1, Perbill::from_parts(618041237)),
        146000 ..= 146999 => (1, Perbill::from_parts(602559726)),
        147000 ..= 147999 => (1, Perbill::from_parts(587288135)),
        148000 ..= 148999 => (1, Perbill::from_parts(572222222)),
        149000 ..= 149999 => (1, Perbill::from_parts(557357859)),
        150000 ..= 150999 => (1, Perbill::from_parts(542691029)),
        151000 ..= 151999 => (1, Perbill::from_parts(528217821)),
        152000 ..= 152999 => (1, Perbill::from_parts(513934426)),
        153000 ..= 153999 => (1, Perbill::from_parts(499837133)),
        154000 ..= 154999 => (1, Perbill::from_parts(485922330)),
        155000 ..= 155999 => (1, Perbill::from_parts(472186495)),
        156000 ..= 156999 => (1, Perbill::from_parts(458626198)),
        157000 ..= 157999 => (1, Perbill::from_parts(445238095)),
        158000 ..= 158999 => (1, Perbill::from_parts(432018927)),
        159000 ..= 159999 => (1, Perbill::from_parts(418965517)),
        160000 ..= 160999 => (1, Perbill::from_parts(406074766)),
        161000 ..= 161999 => (1, Perbill::from_parts(393343653)),
        162000 ..= 162999 => (1, Perbill::from_parts(380769230)),
        163000 ..= 163999 => (1, Perbill::from_parts(368348623)),
        164000 ..= 164999 => (1, Perbill::from_parts(356079027)),
        165000 ..= 165999 => (1, Perbill::from_parts(343957703)),
        166000 ..= 166999 => (1, Perbill::from_parts(331981981)),
        167000 ..= 167999 => (1, Perbill::from_parts(320149253)),
        168000 ..= 168999 => (1, Perbill::from_parts(308456973)),
        169000 ..= 169999 => (1, Perbill::from_parts(296902654)),
        170000 ..= 170999 => (1, Perbill::from_parts(285483870)),
        171000 ..= 171999 => (1, Perbill::from_parts(274198250)),
        172000 ..= 172999 => (1, Perbill::from_parts(263043478)),
        173000 ..= 173999 => (1, Perbill::from_parts(252017291)),
        174000 ..= 174999 => (1, Perbill::from_parts(241117478)),
        175000 ..= 175999 => (1, Perbill::from_parts(230341880)),
        176000 ..= 176999 => (1, Perbill::from_parts(219688385)),
        177000 ..= 177999 => (1, Perbill::from_parts(209154929)),
        178000 ..= 178999 => (1, Perbill::from_parts(198739495)),
        179000 ..= 179999 => (1, Perbill::from_parts(188440111)),
        180000 ..= 180999 => (1, Perbill::from_parts(178254847)),
        181000 ..= 181999 => (1, Perbill::from_parts(168181818)),
        182000 ..= 182999 => (1, Perbill::from_parts(158219178)),
        183000 ..= 183999 => (1, Perbill::from_parts(148365122)),
        184000 ..= 184999 => (1, Perbill::from_parts(138617886)),
        185000 ..= 185999 => (1, Perbill::from_parts(128975741)),
        186000 ..= 186999 => (1, Perbill::from_parts(119436997)),
        187000 ..= 187999 => (1, Perbill::from_parts(109999999)),
        188000 ..= 188999 => (1, Perbill::from_parts(100663129)),
        189000 ..= 189999 => (1, Perbill::from_parts(91424802)),
        190000 ..= 190999 => (1, Perbill::from_parts(82283464)),
        191000 ..= 191999 => (1, Perbill::from_parts(73237597)),
        192000 ..= 192999 => (1, Perbill::from_parts(64285714)),
        193000 ..= 193999 => (1, Perbill::from_parts(55426356)),
        194000 ..= 194999 => (1, Perbill::from_parts(46658097)),
        195000 ..= 195999 => (1, Perbill::from_parts(37979539)),
        196000 ..= 196999 => (1, Perbill::from_parts(29389312)),
        197000 ..= 197999 => (1, Perbill::from_parts(20886075)),
        198000 ..= 198999 => (1, Perbill::from_parts(12468513)),
        199000 ..= 199999 => (1, Perbill::from_parts(4135338)),
        200000 ..= 200999 => (0, Perbill::from_parts(995885286)),
        201000 ..= 201999 => (0, Perbill::from_parts(987717121)),
        202000 ..= 202999 => (0, Perbill::from_parts(979629629)),
        203000 ..= 203999 => (0, Perbill::from_parts(971621621)),
        204000 ..= 204999 => (0, Perbill::from_parts(963691931)),
        205000 ..= 205999 => (0, Perbill::from_parts(955839416)),
        206000 ..= 206999 => (0, Perbill::from_parts(948062953)),
        207000 ..= 207999 => (0, Perbill::from_parts(940361445)),
        208000 ..= 208999 => (0, Perbill::from_parts(932733812)),
        209000 ..= 209999 => (0, Perbill::from_parts(925178997)),
        210000 ..= 210999 => (0, Perbill::from_parts(917695961)),
        211000 ..= 211999 => (0, Perbill::from_parts(910283687)),
        212000 ..= 212999 => (0, Perbill::from_parts(902941176)),
        213000 ..= 213999 => (0, Perbill::from_parts(895667447)),
        214000 ..= 214999 => (0, Perbill::from_parts(888461538)),
        215000 ..= 215999 => (0, Perbill::from_parts(881322505)),
        216000 ..= 216999 => (0, Perbill::from_parts(874249422)),
        217000 ..= 217999 => (0, Perbill::from_parts(867241379)),
        218000 ..= 218999 => (0, Perbill::from_parts(860297482)),
        219000 ..= 219999 => (0, Perbill::from_parts(853416856)),
        220000 ..= 220999 => (0, Perbill::from_parts(846598639)),
        221000 ..= 221999 => (0, Perbill::from_parts(839841986)),
        222000 ..= 222999 => (0, Perbill::from_parts(833146067)),
        223000 ..= 223999 => (0, Perbill::from_parts(826510067)),
        224000 ..= 224999 => (0, Perbill::from_parts(819933184)),
        225000 ..= 225999 => (0, Perbill::from_parts(813414634)),
        226000 ..= 226999 => (0, Perbill::from_parts(806953642)),
        227000 ..= 227999 => (0, Perbill::from_parts(800549450)),
        228000 ..= 228999 => (0, Perbill::from_parts(794201312)),
        229000 ..= 229999 => (0, Perbill::from_parts(787908496)),
        230000 ..= 230999 => (0, Perbill::from_parts(781670281)),
        231000 ..= 231999 => (0, Perbill::from_parts(775485961)),
        232000 ..= 232999 => (0, Perbill::from_parts(769354838)),
        233000 ..= 233999 => (0, Perbill::from_parts(763276231)),
        234000 ..= 234999 => (0, Perbill::from_parts(757249466)),
        235000 ..= 235999 => (0, Perbill::from_parts(751273885)),
        236000 ..= 236999 => (0, Perbill::from_parts(745348837)),
        237000 ..= 237999 => (0, Perbill::from_parts(739473684)),
        238000 ..= 238999 => (0, Perbill::from_parts(733647798)),
        239000 ..= 239999 => (0, Perbill::from_parts(727870563)),
        240000 ..= 240999 => (0, Perbill::from_parts(722141372)),
        241000 ..= 241999 => (0, Perbill::from_parts(716459627)),
        242000 ..= 242999 => (0, Perbill::from_parts(710824742)),
        243000 ..= 243999 => (0, Perbill::from_parts(705236139)),
        244000 ..= 244999 => (0, Perbill::from_parts(699693251)),
        245000 ..= 245999 => (0, Perbill::from_parts(694195519)),
        246000 ..= 246999 => (0, Perbill::from_parts(688742393)),
        247000 ..= 247999 => (0, Perbill::from_parts(683333333)),
        248000 ..= 248999 => (0, Perbill::from_parts(677967806)),
        249000 ..= 249999 => (0, Perbill::from_parts(672645290)),
        250000 ..= 250999 => (0, Perbill::from_parts(667365269)),
        251000 ..= 251999 => (0, Perbill::from_parts(662127236)),
        252000 ..= 252999 => (0, Perbill::from_parts(656930693)),
        253000 ..= 253999 => (0, Perbill::from_parts(651775147)),
        254000 ..= 254999 => (0, Perbill::from_parts(646660117)),
        255000 ..= 255999 => (0, Perbill::from_parts(641585127)),
        256000 ..= 256999 => (0, Perbill::from_parts(636549707)),
        257000 ..= 257999 => (0, Perbill::from_parts(631553398)),
        258000 ..= 258999 => (0, Perbill::from_parts(626595744)),
        259000 ..= 259999 => (0, Perbill::from_parts(621676300)),
        260000 ..= 260999 => (0, Perbill::from_parts(616794625)),
        261000 ..= 261999 => (0, Perbill::from_parts(611950286)),
        262000 ..= 262999 => (0, Perbill::from_parts(607142857)),
        263000 ..= 263999 => (0, Perbill::from_parts(602371916)),
        264000 ..= 264999 => (0, Perbill::from_parts(597637051)),
        265000 ..= 265999 => (0, Perbill::from_parts(592937853)),
        266000 ..= 266999 => (0, Perbill::from_parts(588273921)),
        267000 ..= 267999 => (0, Perbill::from_parts(583644859)),
        268000 ..= 268999 => (0, Perbill::from_parts(579050279)),
        269000 ..= 269999 => (0, Perbill::from_parts(574489795)),
        270000 ..= 270999 => (0, Perbill::from_parts(569963031)),
        271000 ..= 271999 => (0, Perbill::from_parts(565469613)),
        272000 ..= 272999 => (0, Perbill::from_parts(561009174)),
        273000 ..= 273999 => (0, Perbill::from_parts(556581352)),
        274000 ..= 274999 => (0, Perbill::from_parts(552185792)),
        275000 ..= 275999 => (0, Perbill::from_parts(547822141)),
        276000 ..= 276999 => (0, Perbill::from_parts(543490054)),
        277000 ..= 277999 => (0, Perbill::from_parts(539189189)),
        278000 ..= 278999 => (0, Perbill::from_parts(534919210)),
        279000 ..= 279999 => (0, Perbill::from_parts(530679785)),
        280000 ..= 280999 => (0, Perbill::from_parts(526470588)),
        281000 ..= 281999 => (0, Perbill::from_parts(522291296)),
        282000 ..= 282999 => (0, Perbill::from_parts(518141592)),
        283000 ..= 283999 => (0, Perbill::from_parts(514021164)),
        284000 ..= 284999 => (0, Perbill::from_parts(509929701)),
        285000 ..= 285999 => (0, Perbill::from_parts(505866900)),
        286000 ..= 286999 => (0, Perbill::from_parts(501832460)),
        287000 ..= 287999 => (0, Perbill::from_parts(497826086)),
        288000 ..= 288999 => (0, Perbill::from_parts(493847487)),
        289000 ..= 289999 => (0, Perbill::from_parts(489896373)),
        290000 ..= 290999 => (0, Perbill::from_parts(485972461)),
        291000 ..= 291999 => (0, Perbill::from_parts(482075471)),
        292000 ..= 292999 => (0, Perbill::from_parts(478205128)),
        293000 ..= 293999 => (0, Perbill::from_parts(474361158)),
        294000 ..= 294999 => (0, Perbill::from_parts(470543293)),
        295000 ..= 295999 => (0, Perbill::from_parts(466751269)),
        296000 ..= 296999 => (0, Perbill::from_parts(462984822)),
        297000 ..= 297999 => (0, Perbill::from_parts(459243697)),
        298000 ..= 298999 => (0, Perbill::from_parts(455527638)),
        299000 ..= 299999 => (0, Perbill::from_parts(451836393)),
        300000 ..= 300999 => (0, Perbill::from_parts(448169717)),
        301000 ..= 301999 => (0, Perbill::from_parts(444527363)),
        302000 ..= 302999 => (0, Perbill::from_parts(440909090)),
        303000 ..= 303999 => (0, Perbill::from_parts(437314662)),
        304000 ..= 304999 => (0, Perbill::from_parts(433743842)),
        305000 ..= 305999 => (0, Perbill::from_parts(430196399)),
        306000 ..= 306999 => (0, Perbill::from_parts(426672104)),
        307000 ..= 307999 => (0, Perbill::from_parts(423170731)),
        308000 ..= 308999 => (0, Perbill::from_parts(419692058)),
        309000 ..= 309999 => (0, Perbill::from_parts(416235864)),
        310000 ..= 310999 => (0, Perbill::from_parts(412801932)),
        311000 ..= 311999 => (0, Perbill::from_parts(409390048)),
        312000 ..= 312999 => (0, Perbill::from_parts(406000000)),
        313000 ..= 313999 => (0, Perbill::from_parts(402631578)),
        314000 ..= 314999 => (0, Perbill::from_parts(399284578)),
        315000 ..= 315999 => (0, Perbill::from_parts(395958795)),
        316000 ..= 316999 => (0, Perbill::from_parts(392654028)),
        317000 ..= 317999 => (0, Perbill::from_parts(389370078)),
        318000 ..= 318999 => (0, Perbill::from_parts(386106750)),
        319000 ..= 319999 => (0, Perbill::from_parts(382863849)),
        320000 ..= 320999 => (0, Perbill::from_parts(379641185)),
        321000 ..= 321999 => (0, Perbill::from_parts(376438569)),
        322000 ..= 322999 => (0, Perbill::from_parts(373255813)),
        323000 ..= 323999 => (0, Perbill::from_parts(370092735)),
        324000 ..= 324999 => (0, Perbill::from_parts(366949152)),
        325000 ..= 325999 => (0, Perbill::from_parts(363824884)),
        326000 ..= 326999 => (0, Perbill::from_parts(360719754)),
        327000 ..= 327999 => (0, Perbill::from_parts(357633587)),
        328000 ..= 328999 => (0, Perbill::from_parts(354566210)),
        329000 ..= 329999 => (0, Perbill::from_parts(351517450)),
       _ => (0, Perbill::from_parts(350_000_000))
    }
}
