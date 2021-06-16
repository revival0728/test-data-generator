def process_opt(args) -> dict:
    ret = {
        'filename': '',
        'nftd': 0,
        'error': None
    }
    args = args.strip().split()
    if len(args) != 2:
        ret['error'] = 'Number of arguments error (except 2 get {})'.format(len(args))
        return ret
    try:
        with open(args[0], 'r') as f:
            pass
        ret['filename'] = args[0]
    except FileNotFoundError:
        ret['error'] = 'File not found (no file {})'.format(args[0])
        return ret
    try:
        ret['nftd'] = int(args[1])
    except ValueError:
        ret['error'] = 'Argument 2 must be positive integer (get {})'.format(args[1])
    return ret