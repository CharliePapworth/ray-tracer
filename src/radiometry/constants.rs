/// the values of the standard X(lambda), Y(lambda), and Z(lambda) response curves sampled at 1-nm
///  increments from 360 nm to 830 nm. The wavelengths of the nth sample in the arrays below are given
///  by the nth element of CIE_lambda;

pub const N_CIE_SAMPLES: usize = 471;
pub const CIE_X: [f32; N_CIE_SAMPLES as usize] = [
    // CIE X function values
    0.000_129_900_0,
    0.000_145_847_0,
    0.000_163_802_1,
    0.000_184_003_7,
    0.000_206_690_2,
    0.000_232_100_0,
    0.000_260_728_0,
    0.000_293_075_0,
    0.000_329_388_0,
    0.000_369_914_0,
    0.000_414_900_0,
    0.000_464_158_7,
    0.000_518_986_0,
    0.000_581_854_0,
    0.000_655_234_7,
    0.000_741_600_0,
    0.000_845_029_6,
    0.000_964_526_8,
    0.001_094_949,
    0.001_231_154,
    0.001_368_000,
    0.001_502_050,
    0.001_642_328,
    0.001_802_382,
    0.001_995_757,
    0.002_236_000,
    0.002_535_385,
    0.002_892_603,
    0.003_300_829,
    0.003_753_236,
    0.004_243_000,
    0.004_762_389,
    0.005_330_048,
    0.005_978_712,
    0.006_741_117,
    0.007_650_000,
    0.008_751_373,
    0.010_028_88,
    0.011_421_70,
    0.012_869_01,
    0.014_310_00,
    0.015_704_43,
    0.017_147_44,
    0.018_781_22,
    0.020_748_01,
    0.023_190_00,
    0.026_207_36,
    0.029_782_48,
    0.033_880_92,
    0.038_468_24,
    0.043_510_00,
    0.048_995_60,
    0.055_022_60,
    0.061_718_80,
    0.069_212_00,
    0.077_630_00,
    0.086_958_11,
    0.097_176_72,
    0.108_406_3,
    0.120_767_2,
    0.134_380_0,
    0.149_358_2,
    0.165_395_7,
    0.181_983_1,
    0.198_611_0,
    0.214_770_0,
    0.230_186_8,
    0.244_879_7,
    0.258_777_3,
    0.271_807_9,
    0.283_900_0,
    0.294_943_8,
    0.304_896_5,
    0.313_787_3,
    0.321_645_4,
    0.328_500_0,
    0.334_351_3,
    0.339_210_1,
    0.343_121_3,
    0.346_129_6,
    0.348_280_0,
    0.349_599_9,
    0.350_147_4,
    0.350_013_0,
    0.349_287_0,
    0.348_060_0,
    0.346_373_3,
    0.344_262_4,
    0.341_808_8,
    0.339_094_1,
    0.336_200_0,
    0.333_197_7,
    0.330_041_1,
    0.326_635_7,
    0.322_886_8,
    0.318_700_0,
    0.314_025_1,
    0.308_884_0,
    0.303_290_4,
    0.297_257_9,
    0.290_800_0,
    0.283_970_1,
    0.276_721_4,
    0.268_917_8,
    0.260_422_7,
    0.251_100_0,
    0.240_847_5,
    0.229_851_2,
    0.218_407_2,
    0.206_811_5,
    0.195_360_0,
    0.184_213_6,
    0.173_327_3,
    0.162_688_1,
    0.152_283_3,
    0.142_100_0,
    0.132_178_6,
    0.122_569_6,
    0.113_275_2,
    0.104_297_9,
    0.095_640_00,
    0.087_299_55,
    0.079_308_04,
    0.071_717_76,
    0.064_580_99,
    0.057_950_01,
    0.051_862_11,
    0.046_281_52,
    0.041_150_88,
    0.036_412_83,
    0.032_010_00,
    0.027_917_20,
    0.024_144_40,
    0.020_687_00,
    0.017_540_40,
    0.014_700_00,
    0.012_161_79,
    0.009_919_960,
    0.007_967_240,
    0.006_296_346,
    0.004_900_000,
    0.003_777_173,
    0.002_945_320,
    0.002_424_880,
    0.002_236_293,
    0.002_400_000,
    0.002_925_520,
    0.003_836_560,
    0.005_174_840,
    0.006_982_080,
    0.009_300_000,
    0.012_149_49,
    0.015_535_88,
    0.019_477_52,
    0.023_992_77,
    0.029_100_00,
    0.034_814_85,
    0.041_120_16,
    0.047_985_04,
    0.055_378_61,
    0.063_270_00,
    0.071_635_01,
    0.080_462_24,
    0.089_739_96,
    0.099_456_45,
    0.109_600_0,
    0.120_167_4,
    0.131_114_5,
    0.142_367_9,
    0.153_854_2,
    0.165_500_0,
    0.177_257_1,
    0.189_140_0,
    0.201_169_4,
    0.213_365_8,
    0.225_749_9,
    0.238_320_9,
    0.251_066_8,
    0.263_992_2,
    0.277_101_7,
    0.290_400_0,
    0.303_891_2,
    0.317_572_6,
    0.331_438_4,
    0.345_482_8,
    0.359_700_0,
    0.374_083_9,
    0.388_639_6,
    0.403_378_4,
    0.418_311_5,
    0.433_449_9,
    0.448_795_3,
    0.464_336_0,
    0.480_064_0,
    0.495_971_3,
    0.512_050_1,
    0.528_295_9,
    0.544_691_6,
    0.561_209_4,
    0.577_821_5,
    0.594_500_0,
    0.611_220_9,
    0.627_975_8,
    0.644_760_2,
    0.661_569_7,
    0.678_400_0,
    0.695_239_2,
    0.712_058_6,
    0.728_828_4,
    0.745_518_8,
    0.762_100_0,
    0.778_543_2,
    0.794_825_6,
    0.810_926_4,
    0.826_824_8,
    0.842_500_0,
    0.857_932_5,
    0.873_081_6,
    0.887_894_4,
    0.902_318_1,
    0.916_300_0,
    0.929_799_5,
    0.942_798_4,
    0.955_277_6,
    0.967_217_9,
    0.978_600_0,
    0.989_385_6,
    0.999_548_8,
    1.009_089_2,
    1.018_006_4,
    1.026_300_0,
    1.033_982_7,
    1.040_986_0,
    1.047_188_0,
    1.052_466_7,
    1.056_700_0,
    1.059_794_4,
    1.061_799_2,
    1.062_806_8,
    1.062_909_6,
    1.062_200_0,
    1.060_735_2,
    1.058_443_6,
    1.055_224_4,
    1.050_976_8,
    1.045_600_0,
    1.039_036_9,
    1.031_360_8,
    1.022_666_2,
    1.013_047_7,
    1.002_600_0,
    0.991_367_5,
    0.979_331_4,
    0.966_491_6,
    0.952_847_9,
    0.938_400_0,
    0.923_194_0,
    0.907_244_0,
    0.890_502_0,
    0.872_920_0,
    0.854_449_9,
    0.835_084_0,
    0.814_946_0,
    0.794_186_0,
    0.772_954_0,
    0.751_400_0,
    0.729_583_6,
    0.707_588_8,
    0.685_602_2,
    0.663_810_4,
    0.642_400_0,
    0.621_514_9,
    0.601_113_8,
    0.581_105_2,
    0.561_397_7,
    0.541_900_0,
    0.522_599_5,
    0.503_546_4,
    0.484_743_6,
    0.466_193_9,
    0.447_900_0,
    0.429_861_3,
    0.412_098_0,
    0.394_644_0,
    0.377_533_3,
    0.360_800_0,
    0.344_456_3,
    0.328_516_8,
    0.313_019_2,
    0.298_001_1,
    0.283_500_0,
    0.269_544_8,
    0.256_118_4,
    0.243_189_6,
    0.230_727_2,
    0.218_700_0,
    0.207_097_1,
    0.195_923_2,
    0.185_170_8,
    0.174_832_3,
    0.164_900_0,
    0.155_366_7,
    0.146_230_0,
    0.137_490_0,
    0.129_146_7,
    0.121_200_0,
    0.113_639_7,
    0.106_465_0,
    0.099_690_44,
    0.093_330_61,
    0.087_400_00,
    0.081_900_96,
    0.076_804_28,
    0.072_077_12,
    0.067_686_640,
    0.063_600_00,
    0.059_806_85,
    0.056_282_16,
    0.052_971_04,
    0.049_818_61,
    0.046_770_00,
    0.043_784_05,
    0.040_875_36,
    0.038_072_640,
    0.035_404_61,
    0.032_900_00,
    0.030_564_19,
    0.028_380_56,
    0.026_344_84,
    0.024_452_75,
    0.022_700_00,
    0.021_084_29,
    0.019_599_88,
    0.018_237_320,
    0.016_987_17,
    0.015_840_00,
    0.014_790_640,
    0.013_831_320,
    0.012_948_68,
    0.012_129_20,
    0.011_359_16,
    0.010_629_35,
    0.009_938_846,
    0.009_288_422,
    0.008_678_854,
    0.008_110_916,
    0.007_582_388,
    0.007_088_746,
    0.006_627_313,
    0.006_195_408,
    0.005_790_346,
    0.005_409_826,
    0.005_052_583,
    0.004_717_512,
    0.004_403_507,
    0.004_109_457,
    0.003_833_913,
    0.003_575_748,
    0.003_334_342,
    0.003_109_075,
    0.002_899_327,
    0.002_704_348,
    0.002_523_020,
    0.002_354_168,
    0.002_196_616,
    0.002_049_190,
    0.001_910_960,
    0.001_781_438,
    0.001_660_110,
    0.001_546_459,
    0.001_439_971,
    0.001_340_042,
    0.001_246_275,
    0.001_158_471,
    0.001_076_430,
    0.000_999_949_3,
    0.000_928_735_8,
    0.000_862_433_2,
    0.000_800_750_3,
    0.000_743_396_0,
    0.000_690_078_6,
    0.000_640_515_6,
    0.000_594_502_1,
    0.000_551_864_6,
    0.000_512_429_0,
    0.000_476_021_3,
    0.000_442_453_6,
    0.000_411_511_7,
    0.000_382_981_4,
    0.000_356_649_1,
    0.000_332_301_1,
    0.000_309_758_6,
    0.000_288_887_1,
    0.000_269_539_4,
    0.000_251_568_2,
    0.000_234_826_1,
    0.000_219_171_0,
    0.000_204_525_8,
    0.000_190_840_5,
    0.000_178_065_4,
    0.000_166_150_5,
    0.000_155_023_6,
    0.000_144_621_9,
    0.000_134_909_8,
    0.000_125_852_0,
    0.000_117_413_0,
    0.000_109_551_5,
    0.000_102_224_5,
    0.000_095_394_45,
    0.000_089_023_90,
    0.000_083_075_27,
    0.000_077_512_69,
    0.000_072_313_04,
    0.000_067_457_78,
    0.000_062_928_44,
    0.000_058_706_52,
    0.000_054_770_28,
    0.000_051_099_18,
    0.000_047_676_54,
    0.000_044_485_67,
    0.000_041_509_94,
    0.000_038_733_24,
    0.000_036_142_03,
    0.000_033_723_52,
    0.000_031_464_87,
    0.000_029_353_26,
    0.000_027_375_73,
    0.000_025_524_33,
    0.000_023_793_76,
    0.000_022_178_70,
    0.000_020_673_83,
    0.000_019_272_26,
    0.000_017_966_40,
    0.000_016_749_91,
    0.000_015_616_48,
    0.000_014_559_77,
    0.000_013_573_87,
    0.000_012_654_36,
    0.000_011_797_23,
    0.000_010_998_44,
    0.000_010_253_98,
    0.000_009_559_646,
    0.000_008_912_044,
    0.000_008_308_358,
    0.000_007_745_769,
    0.000_007_221_456,
    0.000_006_732_475,
    0.000_006_276_423,
    0.000_005_851_304,
    0.000_005_455_118,
    0.000_005_085_868,
    0.000_004_741_466,
    0.000_004_420_236,
    0.000_004_120_783,
    0.000_003_841_716,
    0.000_003_581_652,
    0.000_003_339_127,
    0.000_003_112_949,
    0.000_002_902_121,
    0.000_002_705_645,
    0.000_002_522_525,
    0.000_002_351_726,
    0.000_002_192_415,
    0.000_002_043_902,
    0.000_001_905_497,
    0.000_001_776_509,
    0.000_001_656_215,
    0.000_001_544_022,
    0.000_001_439_440,
    0.000_001_341_977,
    0.000_001_251_141,
];

pub const CIE_Y: [f32; N_CIE_SAMPLES as usize] = [
    // CIE Y function values
    0.000_003_917_000,
    0.000_004_393_581,
    0.000_004_929_604,
    0.000_005_532_136,
    0.000_006_208_245,
    0.000_006_965_000,
    0.000_007_813_219,
    0.000_008_767_336,
    0.000_009_839_844,
    0.000_011_043_23,
    0.000_012_390_00,
    0.000_013_886_41,
    0.000_015_557_28,
    0.000_017_442_96,
    0.000_019_583_75,
    0.000_022_020_00,
    0.000_024_839_65,
    0.000_028_041_26,
    0.000_031_531_04,
    0.000_035_215_21,
    0.000_039_000_00,
    0.000_042_826_40,
    0.000_046_914_60,
    0.000_051_589_60,
    0.000_057_176_40,
    0.000_064_000_00,
    0.000_072_344_21,
    0.000_082_212_24,
    0.000_093_508_16,
    0.000_106_136_1,
    0.000_120_000_0,
    0.000_134_984_0,
    0.000_151_492_0,
    0.000_170_208_0,
    0.000_191_816_0,
    0.000_217_000_0,
    0.000_246_906_7,
    0.000_281_240_0,
    0.000_318_520_0,
    0.000_357_266_7,
    0.000_396_000_0,
    0.000_433_714_7,
    0.000_473_024_0,
    0.000_517_876_0,
    0.000_572_218_7,
    0.000_640_000_0,
    0.000_724_560_0,
    0.000_825_500_0,
    0.000_941_160_0,
    0.001_069_880,
    0.001_210_000,
    0.001_362_091,
    0.001_530_752,
    0.001_720_368,
    0.001_935_323,
    0.002_180_000,
    0.002_454_800,
    0.002_764_000,
    0.003_117_800,
    0.003_526_400,
    0.004_000_000,
    0.004_546_240,
    0.005_159_320,
    0.005_829_280,
    0.006_546_160,
    0.007_300_000,
    0.008_086_507,
    0.008_908_720,
    0.009_767_680,
    0.010_664_43,
    0.011_600_00,
    0.012_573_17,
    0.013_582_72,
    0.014_629_68,
    0.015_715_09,
    0.016_840_00,
    0.018_007_36,
    0.019_214_48,
    0.020_453_92,
    0.021_718_24,
    0.023_000_00,
    0.024_294_61,
    0.025_610_24,
    0.026_958_57,
    0.028_351_25,
    0.029_800_00,
    0.031_310_83,
    0.032_883_68,
    0.034_521_12,
    0.036_225_71,
    0.038_000_00,
    0.039_846_67,
    0.041_768_00,
    0.043_766_00,
    0.045_842_67,
    0.048_000_00,
    0.050_243_68,
    0.052_573_04,
    0.054_980_56,
    0.057_458_72,
    0.060_000_00,
    0.062_601_97,
    0.065_277_52,
    0.068_042_08,
    0.070_911_09,
    0.073_900_00,
    0.077_016_00,
    0.080_266_40,
    0.083_666_80,
    0.087_232_80,
    0.090_980_00,
    0.094_917_55,
    0.099_045_84,
    0.103_367_4,
    0.107_884_6,
    0.112_600_0,
    0.117_532_0,
    0.122_674_4,
    0.127_992_8,
    0.133_452_8,
    0.139_020_0,
    0.144_676_4,
    0.150_469_3,
    0.156_461_9,
    0.162_717_7,
    0.169_300_0,
    0.176_243_1,
    0.183_558_1,
    0.191_273_5,
    0.199_418_0,
    0.208_020_0,
    0.217_119_9,
    0.226_734_5,
    0.236_857_1,
    0.247_481_2,
    0.258_600_0,
    0.270_184_9,
    0.282_293_9,
    0.295_050_5,
    0.308_578_0,
    0.323_000_0,
    0.338_402_1,
    0.354_685_8,
    0.371_698_6,
    0.389_287_5,
    0.407_300_0,
    0.425_629_9,
    0.444_309_6,
    0.463_394_4,
    0.482_939_5,
    0.503_000_0,
    0.523_569_3,
    0.544_512_0,
    0.565_690_0,
    0.586_965_3,
    0.608_200_0,
    0.629_345_6,
    0.650_306_8,
    0.670_875_2,
    0.690_842_4,
    0.710_000_0,
    0.728_185_2,
    0.745_463_6,
    0.761_969_4,
    0.777_836_8,
    0.793_200_0,
    0.808_110_4,
    0.822_496_2,
    0.836_306_8,
    0.849_491_6,
    0.862_000_0,
    0.873_810_8,
    0.884_962_4,
    0.895_493_6,
    0.905_443_2,
    0.914_850_1,
    0.923_734_8,
    0.932_092_4,
    0.939_922_6,
    0.947_225_2,
    0.954_000_0,
    0.960_256_1,
    0.966_007_4,
    0.971_260_6,
    0.976_022_5,
    0.980_300_0,
    0.984_092_4,
    0.987_481_2,
    0.990_312_8,
    0.992_811_6,
    0.994_950_1,
    0.996_710_8,
    0.998_098_3,
    0.999_112_0,
    0.999_748_2,
    1.000_000_0,
    0.999_856_7,
    0.999_304_6,
    0.998_325_5,
    0.996_898_7,
    0.995_000_0,
    0.992_600_5,
    0.989_742_6,
    0.986_444_4,
    0.982_724_1,
    0.978_600_0,
    0.974_083_7,
    0.969_171_2,
    0.963_856_8,
    0.958_134_9,
    0.952_000_0,
    0.945_450_4,
    0.938_499_2,
    0.931_162_8,
    0.923_457_6,
    0.915_400_0,
    0.907_006_4,
    0.898_277_2,
    0.889_204_8,
    0.879_781_6,
    0.870_000_0,
    0.859_861_3,
    0.849_392_0,
    0.838_622_0,
    0.827_581_3,
    0.816_300_0,
    0.804_794_7,
    0.793_082_0,
    0.781_192_0,
    0.769_154_7,
    0.757_000_0,
    0.744_754_1,
    0.732_422_4,
    0.720_003_6,
    0.707_496_5,
    0.694_900_0,
    0.682_219_2,
    0.669_471_6,
    0.656_674_4,
    0.643_844_8,
    0.631_000_0,
    0.618_155_5,
    0.605_314_4,
    0.592_475_6,
    0.579_637_9,
    0.566_800_0,
    0.553_961_1,
    0.541_137_2,
    0.528_352_8,
    0.515_632_3,
    0.503_000_0,
    0.490_468_8,
    0.478_030_4,
    0.465_677_6,
    0.453_403_2,
    0.441_200_0,
    0.429_080_0,
    0.417_036_0,
    0.405_032_0,
    0.393_032_0,
    0.381_000_0,
    0.368_918_4,
    0.356_827_2,
    0.344_776_8,
    0.332_817_6,
    0.321_000_0,
    0.309_338_1,
    0.297_850_4,
    0.286_593_6,
    0.275_624_5,
    0.265_000_0,
    0.254_763_2,
    0.244_889_6,
    0.235_334_4,
    0.226_052_8,
    0.217_000_0,
    0.208_161_6,
    0.199_548_8,
    0.191_155_2,
    0.182_974_4,
    0.175_000_0,
    0.167_223_5,
    0.159_646_4,
    0.152_277_6,
    0.145_125_9,
    0.138_200_0,
    0.131_500_3,
    0.125_024_8,
    0.118_779_2,
    0.112_769_1,
    0.107_000_0,
    0.101_476_2,
    0.096_188_640,
    0.091_122_96,
    0.086_264_85,
    0.081_600_00,
    0.077_120_640,
    0.072_825_52,
    0.068_710_08,
    0.064_769_76,
    0.061_000_00,
    0.057_396_21,
    0.053_955_04,
    0.050_673_76,
    0.047_549_65,
    0.044_580_00,
    0.041_758_72,
    0.039_084_96,
    0.036_563_84,
    0.034_200_48,
    0.032_000_00,
    0.029_962_61,
    0.028_076_640,
    0.026_329_36,
    0.024_708_05,
    0.023_200_00,
    0.021_800_77,
    0.020_501_12,
    0.019_281_08,
    0.018_120_69,
    0.017_000_00,
    0.015_903_79,
    0.014_837_18,
    0.013_810_68,
    0.012_834_78,
    0.011_920_00,
    0.011_068_31,
    0.010_273_39,
    0.009_533_311,
    0.008_846_157,
    0.008_210_000,
    0.007_623_781,
    0.007_085_424,
    0.006_591_476,
    0.006_138_485,
    0.005_723_000,
    0.005_343_059,
    0.004_995_796,
    0.004_676_404,
    0.004_380_075,
    0.004_102_000,
    0.003_838_453,
    0.003_589_099,
    0.003_354_219,
    0.003_134_093,
    0.002_929_000,
    0.002_738_139,
    0.002_559_876,
    0.002_393_244,
    0.002_237_275,
    0.002_091_000,
    0.001_953_587,
    0.001_824_580,
    0.001_703_580,
    0.001_590_187,
    0.001_484_000,
    0.001_384_496,
    0.001_291_268,
    0.001_204_092,
    0.001_122_744,
    0.001_047_000,
    0.000_976_589_6,
    0.000_911_108_8,
    0.000_850_133_2,
    0.000_793_238_4,
    0.000_740_000_0,
    0.000_690_082_7,
    0.000_643_310_0,
    0.000_599_496_0,
    0.000_558_454_7,
    0.000_520_000_0,
    0.000_483_913_6,
    0.000_450_052_8,
    0.000_418_345_2,
    0.000_388_718_4,
    0.000_361_100_0,
    0.000_335_383_5,
    0.000_311_440_4,
    0.000_289_165_6,
    0.000_268_453_9,
    0.000_249_200_0,
    0.000_231_301_9,
    0.000_214_685_6,
    0.000_199_288_4,
    0.000_185_047_5,
    0.000_171_900_0,
    0.000_159_778_1,
    0.000_148_604_4,
    0.000_138_301_6,
    0.000_128_792_5,
    0.000_120_000_0,
    0.000_111_859_5,
    0.000_104_322_4,
    0.000_097_335_60,
    0.000_090_845_87,
    0.000_084_800_00,
    0.000_079_146_67,
    0.000_073_858_00,
    0.000_068_916_00,
    0.000_064_302_67,
    0.000_060_000_00,
    0.000_055_981_87,
    0.000_052_225_60,
    0.000_048_718_40,
    0.000_045_447_47,
    0.000_042_400_00,
    0.000_039_561_04,
    0.000_036_915_12,
    0.000_034_448_68,
    0.000_032_148_16,
    0.000_030_000_00,
    0.000_027_991_25,
    0.000_026_113_56,
    0.000_024_360_24,
    0.000_022_724_61,
    0.000_021_200_00,
    0.000_019_778_55,
    0.000_018_452_85,
    0.000_017_216_87,
    0.000_016_064_59,
    0.000_014_990_00,
    0.000_013_987_28,
    0.000_013_051_55,
    0.000_012_178_18,
    0.000_011_362_54,
    0.000_010_600_00,
    0.000_009_885_877,
    0.000_009_217_304,
    0.000_008_592_362,
    0.000_008_009_133,
    0.000_007_465_700,
    0.000_006_959_567,
    0.000_006_487_995,
    0.000_006_048_699,
    0.000_005_639_396,
    0.000_005_257_800,
    0.000_004_901_771,
    0.000_004_569_720,
    0.000_004_260_194,
    0.000_003_971_739,
    0.000_003_702_900,
    0.000_003_452_163,
    0.000_003_218_302,
    0.000_003_000_300,
    0.000_002_797_139,
    0.000_002_607_800,
    0.000_002_431_220,
    0.000_002_266_531,
    0.000_002_113_013,
    0.000_001_969_943,
    0.000_001_836_600,
    0.000_001_712_230,
    0.000_001_596_228,
    0.000_001_488_090,
    0.000_001_387_314,
    0.000_001_293_400,
    0.000_001_205_820,
    0.000_001_124_143,
    0.000_001_048_009,
    0.000_000_977_057_8,
    0.000_000_910_930_0,
    0.000_000_849_251_3,
    0.000_000_791_721_2,
    0.000_000_738_090_4,
    0.000_000_688_109_8,
    0.000_000_641_530_0,
    0.000_000_598_089_5,
    0.000_000_557_574_6,
    0.000_000_519_808_0,
    0.000_000_484_612_3,
    0.000_000_451_810_0,
];

pub const CIE_Z: [f32; N_CIE_SAMPLES as usize] = [
    // CIE Z function values
    0.000_606_100_0,
    0.000_680_879_2,
    0.000_765_145_6,
    0.000_860_012_4,
    0.000_966_592_8,
    0.001_086_000,
    0.001_220_586,
    0.001_372_729,
    0.001_543_579,
    0.001_734_286,
    0.001_946_000,
    0.002_177_777,
    0.002_435_809,
    0.002_731_953,
    0.003_078_064,
    0.003_486_000,
    0.003_975_227,
    0.004_540_880,
    0.005_158_320,
    0.005_802_907,
    0.006_450_001,
    0.007_083_216,
    0.007_745_488,
    0.008_501_152,
    0.009_414_544,
    0.010_549_99,
    0.011_965_80,
    0.013_655_87,
    0.015_588_05,
    0.017_730_15,
    0.020_050_01,
    0.022_511_36,
    0.025_202_88,
    0.028_279_72,
    0.031_897_04,
    0.036_210_00,
    0.041_437_71,
    0.047_503_72,
    0.054_119_88,
    0.060_998_03,
    0.067_850_01,
    0.074_486_320,
    0.081_361_56,
    0.089_153_640,
    0.098_540_48,
    0.110_200_0,
    0.124_613_3,
    0.141_701_7,
    0.161_303_5,
    0.183_256_8,
    0.207_400_0,
    0.233_692_1,
    0.262_611_4,
    0.294_774_6,
    0.330_798_5,
    0.371_300_0,
    0.416_209_1,
    0.465_464_2,
    0.519_694_8,
    0.579_530_3,
    0.645_600_0,
    0.718_483_8,
    0.796_713_3,
    0.877_845_9,
    0.959_439_0,
    1.039_050_1,
    1.115_367_3,
    1.188_497_1,
    1.258_123_3,
    1.323_929_6,
    1.385_600_0,
    1.442_635_2,
    1.494_803_5,
    1.542_190_3,
    1.584_880_7,
    1.622_960_0,
    1.656_404_8,
    1.685_295_9,
    1.709_874_5,
    1.730_382_1,
    1.747_060_0,
    1.760_044_6,
    1.769_623_3,
    1.776_263_7,
    1.780_433_4,
    1.782_600_0,
    1.782_968_2,
    1.781_699_8,
    1.779_198_2,
    1.775_867_1,
    1.772_110_0,
    1.768_258_9,
    1.764_039_0,
    1.758_943_8,
    1.752_466_3,
    1.744_100_0,
    1.733_559_5,
    1.720_858_1,
    1.705_936_9,
    1.688_737_2,
    1.669_200_0,
    1.647_528_7,
    1.623_412_7,
    1.596_022_3,
    1.564_528_0,
    1.528_100_0,
    1.486_111_4,
    1.439_521_5,
    1.389_879_9,
    1.338_736_2,
    1.287_640_0,
    1.237_422_3,
    1.187_824_3,
    1.138_761_1,
    1.090_148_0,
    1.041_900_0,
    0.994_197_6,
    0.947_347_3,
    0.901_453_1,
    0.856_619_3,
    0.812_950_1,
    0.770_517_3,
    0.729_444_8,
    0.689_913_6,
    0.652_104_9,
    0.616_200_0,
    0.582_328_6,
    0.550_416_2,
    0.520_337_6,
    0.491_967_3,
    0.465_180_0,
    0.439_924_6,
    0.416_183_6,
    0.393_882_2,
    0.372_945_9,
    0.353_300_0,
    0.334_857_8,
    0.317_552_1,
    0.301_337_5,
    0.286_168_6,
    0.272_000_0,
    0.258_817_1,
    0.246_483_8,
    0.234_771_8,
    0.223_453_3,
    0.212_300_0,
    0.201_169_2,
    0.190_119_6,
    0.179_225_4,
    0.168_560_8,
    0.158_200_0,
    0.148_138_3,
    0.138_375_8,
    0.128_994_2,
    0.120_075_1,
    0.111_700_0,
    0.103_904_8,
    0.096_667_48,
    0.089_982_72,
    0.083_845_31,
    0.078_249_99,
    0.073_208_99,
    0.068_678_16,
    0.064_567_84,
    0.060_788_35,
    0.057_250_01,
    0.053_904_35,
    0.050_746_640,
    0.047_752_76,
    0.044_898_59,
    0.042_160_00,
    0.039_507_28,
    0.036_935_640,
    0.034_458_36,
    0.032_088_72,
    0.029_840_00,
    0.027_711_81,
    0.025_694_44,
    0.023_787_16,
    0.021_989_25,
    0.020_300_00,
    0.018_718_05,
    0.017_240_36,
    0.015_863_640,
    0.014_584_61,
    0.013_400_00,
    0.012_307_23,
    0.011_301_88,
    0.010_377_92,
    0.009_529_306,
    0.008_749_999,
    0.008_035_200,
    0.007_381_600,
    0.006_785_400,
    0.006_242_800,
    0.005_749_999,
    0.005_303_600,
    0.004_899_800,
    0.004_534_200,
    0.004_202_400,
    0.003_900_000,
    0.003_623_200,
    0.003_370_600,
    0.003_141_400,
    0.002_934_800,
    0.002_749_999,
    0.002_585_200,
    0.002_438_600,
    0.002_309_400,
    0.002_196_800,
    0.002_100_000,
    0.002_017_733,
    0.001_948_200,
    0.001_889_800,
    0.001_840_933,
    0.001_800_000,
    0.001_766_267,
    0.001_737_800,
    0.001_711_200,
    0.001_683_067,
    0.001_650_001,
    0.001_610_133,
    0.001_564_400,
    0.001_513_600,
    0.001_458_533,
    0.001_400_000,
    0.001_336_667,
    0.001_270_000,
    0.001_205_000,
    0.001_146_667,
    0.001_100_000,
    0.001_068_800,
    0.001_049_400,
    0.001_035_600,
    0.001_021_200,
    0.001_000_000,
    0.000_968_640_0,
    0.000_929_920_0,
    0.000_886_880_0,
    0.000_842_560_0,
    0.000_800_000_0,
    0.000_760_960_0,
    0.000_723_680_0,
    0.000_685_920_0,
    0.000_645_440_0,
    0.000_600_000_0,
    0.000_547_866_7,
    0.000_491_600_0,
    0.000_435_400_0,
    0.000_383_466_7,
    0.000_340_000_0,
    0.000_307_253_3,
    0.000_283_160_0,
    0.000_265_440_0,
    0.000_251_813_3,
    0.000_240_000_0,
    0.000_229_546_7,
    0.000_220_640_0,
    0.000_211_960_0,
    0.000_202_186_7,
    0.000_190_000_0,
    0.000_174_213_3,
    0.000_155_640_0,
    0.000_135_960_0,
    0.000_116_853_3,
    0.000_100_000_0,
    0.000_086_133_33,
    0.000_074_600_00,
    0.000_065_000_00,
    0.000_056_933_33,
    0.000_049_999_99,
    0.000_044_160_00,
    0.000_039_480_00,
    0.000_035_720_00,
    0.000_032_640_00,
    0.000_030_000_00,
    0.000_027_653_33,
    0.000_025_560_00,
    0.000_023_640_00,
    0.000_021_813_33,
    0.000_020_000_00,
    0.000_018_133_33,
    0.000_016_200_00,
    0.000_014_200_00,
    0.000_012_133_33,
    0.000_010_000_00,
    0.000_007_733_333,
    0.000_005_400_000,
    0.000_003_200_000,
    0.000_001_333_333,
    0.000_000_000_000,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
];

pub const CIE_LAMBDA: [f32; N_CIE_SAMPLES as usize] = [
    360.0, 361.0, 362.0, 363.0, 364.0, 365.0, 366.0, 367.0, 368.0, 369.0, 370.0, 371.0, 372.0,
    373.0, 374.0, 375.0, 376.0, 377.0, 378.0, 379.0, 380.0, 381.0, 382.0, 383.0, 384.0, 385.0,
    386.0, 387.0, 388.0, 389.0, 390.0, 391.0, 392.0, 393.0, 394.0, 395.0, 396.0, 397.0, 398.0,
    399.0, 400.0, 401.0, 402.0, 403.0, 404.0, 405.0, 406.0, 407.0, 408.0, 409.0, 410.0, 411.0,
    412.0, 413.0, 414.0, 415.0, 416.0, 417.0, 418.0, 419.0, 420.0, 421.0, 422.0, 423.0, 424.0,
    425.0, 426.0, 427.0, 428.0, 429.0, 430.0, 431.0, 432.0, 433.0, 434.0, 435.0, 436.0, 437.0,
    438.0, 439.0, 440.0, 441.0, 442.0, 443.0, 444.0, 445.0, 446.0, 447.0, 448.0, 449.0, 450.0,
    451.0, 452.0, 453.0, 454.0, 455.0, 456.0, 457.0, 458.0, 459.0, 460.0, 461.0, 462.0, 463.0,
    464.0, 465.0, 466.0, 467.0, 468.0, 469.0, 470.0, 471.0, 472.0, 473.0, 474.0, 475.0, 476.0,
    477.0, 478.0, 479.0, 480.0, 481.0, 482.0, 483.0, 484.0, 485.0, 486.0, 487.0, 488.0, 489.0,
    490.0, 491.0, 492.0, 493.0, 494.0, 495.0, 496.0, 497.0, 498.0, 499.0, 500.0, 501.0, 502.0,
    503.0, 504.0, 505.0, 506.0, 507.0, 508.0, 509.0, 510.0, 511.0, 512.0, 513.0, 514.0, 515.0,
    516.0, 517.0, 518.0, 519.0, 520.0, 521.0, 522.0, 523.0, 524.0, 525.0, 526.0, 527.0, 528.0,
    529.0, 530.0, 531.0, 532.0, 533.0, 534.0, 535.0, 536.0, 537.0, 538.0, 539.0, 540.0, 541.0,
    542.0, 543.0, 544.0, 545.0, 546.0, 547.0, 548.0, 549.0, 550.0, 551.0, 552.0, 553.0, 554.0,
    555.0, 556.0, 557.0, 558.0, 559.0, 560.0, 561.0, 562.0, 563.0, 564.0, 565.0, 566.0, 567.0,
    568.0, 569.0, 570.0, 571.0, 572.0, 573.0, 574.0, 575.0, 576.0, 577.0, 578.0, 579.0, 580.0,
    581.0, 582.0, 583.0, 584.0, 585.0, 586.0, 587.0, 588.0, 589.0, 590.0, 591.0, 592.0, 593.0,
    594.0, 595.0, 596.0, 597.0, 598.0, 599.0, 600.0, 601.0, 602.0, 603.0, 604.0, 605.0, 606.0,
    607.0, 608.0, 609.0, 610.0, 611.0, 612.0, 613.0, 614.0, 615.0, 616.0, 617.0, 618.0, 619.0,
    620.0, 621.0, 622.0, 623.0, 624.0, 625.0, 626.0, 627.0, 628.0, 629.0, 630.0, 631.0, 632.0,
    633.0, 634.0, 635.0, 636.0, 637.0, 638.0, 639.0, 640.0, 641.0, 642.0, 643.0, 644.0, 645.0,
    646.0, 647.0, 648.0, 649.0, 650.0, 651.0, 652.0, 653.0, 654.0, 655.0, 656.0, 657.0, 658.0,
    659.0, 660.0, 661.0, 662.0, 663.0, 664.0, 665.0, 666.0, 667.0, 668.0, 669.0, 670.0, 671.0,
    672.0, 673.0, 674.0, 675.0, 676.0, 677.0, 678.0, 679.0, 680.0, 681.0, 682.0, 683.0, 684.0,
    685.0, 686.0, 687.0, 688.0, 689.0, 690.0, 691.0, 692.0, 693.0, 694.0, 695.0, 696.0, 697.0,
    698.0, 699.0, 700.0, 701.0, 702.0, 703.0, 704.0, 705.0, 706.0, 707.0, 708.0, 709.0, 710.0,
    711.0, 712.0, 713.0, 714.0, 715.0, 716.0, 717.0, 718.0, 719.0, 720.0, 721.0, 722.0, 723.0,
    724.0, 725.0, 726.0, 727.0, 728.0, 729.0, 730.0, 731.0, 732.0, 733.0, 734.0, 735.0, 736.0,
    737.0, 738.0, 739.0, 740.0, 741.0, 742.0, 743.0, 744.0, 745.0, 746.0, 747.0, 748.0, 749.0,
    750.0, 751.0, 752.0, 753.0, 754.0, 755.0, 756.0, 757.0, 758.0, 759.0, 760.0, 761.0, 762.0,
    763.0, 764.0, 765.0, 766.0, 767.0, 768.0, 769.0, 770.0, 771.0, 772.0, 773.0, 774.0, 775.0,
    776.0, 777.0, 778.0, 779.0, 780.0, 781.0, 782.0, 783.0, 784.0, 785.0, 786.0, 787.0, 788.0,
    789.0, 790.0, 791.0, 792.0, 793.0, 794.0, 795.0, 796.0, 797.0, 798.0, 799.0, 800.0, 801.0,
    802.0, 803.0, 804.0, 805.0, 806.0, 807.0, 808.0, 809.0, 810.0, 811.0, 812.0, 813.0, 814.0,
    815.0, 816.0, 817.0, 818.0, 819.0, 820.0, 821.0, 822.0, 823.0, 824.0, 825.0, 826.0, 827.0,
    828.0, 829.0, 830.0,
];

pub const CIE_Y_INTEGRAL: f32 = 106.856_895;