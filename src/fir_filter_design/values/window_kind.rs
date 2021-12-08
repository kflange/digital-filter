pub enum WindowKind {
    Hamming,
    Kaiser(f64),
}
//_win_equiv_raw = {
//    ('barthann', 'brthan', 'bth'): (barthann, False),
//    ('bartlett', 'bart', 'brt'): (bartlett, False),
//    ('blackman', 'black', 'blk'): (blackman, False),
//    ('blackmanharris', 'blackharr', 'bkh'): (blackmanharris, False),
//    ('bohman', 'bman', 'bmn'): (bohman, False),
//    ('boxcar', 'box', 'ones',
//        'rect', 'rectangular'): (boxcar, False),
//    ('chebwin', 'cheb'): (chebwin, True),
//    ('cosine', 'halfcosine'): (cosine, False),
//    ('dpss',): (dpss, True),
//    ('exponential', 'poisson'): (exponential, True),
//    ('flattop', 'flat', 'flt'): (flattop, False),
//    ('gaussian', 'gauss', 'gss'): (gaussian, True),
//    ('general gaussian', 'general_gaussian',
//        'general gauss', 'general_gauss', 'ggs'): (general_gaussian, True),
//    ('hamming', 'hamm', 'ham'): (hamming, False),
//    ('hanning', 'hann', 'han'): (hann, False),
//    ('kaiser', 'ksr'): (kaiser, True),
//    ('nuttall', 'nutl', 'nut'): (nuttall, False),
//    ('parzen', 'parz', 'par'): (parzen, False),
//    ('taylor', 'taylorwin'): (taylor, False),
//    ('triangle', 'triang', 'tri'): (triang, False),
//    ('tukey', 'tuk'): (tukey, True),
//}
