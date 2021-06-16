class compiler:
    def __init__(self):
        self.sp_symbol = [('[', ']'), ('{', '}'), ('(', ')'), ('begin', 'end')]
        self.sp_word = ['set', 'int', 'float', 'list', '/', 'main', 'nftd', 'fnp', 'as']
        self.sp_var = ['ENDL']
        self.ret = {
            'nftd': '',
            'fnp': '',
            'error': None,
            'exec': []  # ('format_string', (types), (ranges), times)
        }
        self.flag = 0
        self.var_name = []
        self.var = []

    def process_sp_word(self, words) -> tuple:
        if words == ('begin', 'main'):
            self.flag += 1
        elif words[0] == 'begin' and words[2] == 'as':
            buf = words[1].split(',')
            for i in buf:
                i = i.strip()
                if not i in self.var_name:
                    self.ret['error'] = 'Variable {} is not declared in this scope'.format(i)
                    return 'get_error', ()
            var = words[3].split(',')
            for i in var: i = i.strip()
            if not len(var) == len(buf):
                self.ret['error'] = 'Cannot unpack tuple (varibale expect {} get {})'.format(len(buf), len(var))
                return 'get_error', ()
            return 'accpet', tuple(var)
        elif words[0] == 'set':
            if not len(words) == 3:
                self.ret['error'] = 'Sytanx error (argument expect 2 get {})'.format(len(words)-1)
                return 'get_error', ()
            if not words[1] in ('nftd', 'fnp'):
                self.ret['error'] = 'Sytanx error (argument {} not found)'.format(words[1])
                return 'get_error', ()
            if words[1] == 'nftd':
                try:
                    x = int(words[2])
                except ValueError:
                    self.ret['error'] = 'Value error (cannot convert {} to positive integer)'.format(words[2])
                    return 'get_error', ()
            elif words[1] == 'fnp':
                if not '{}' in words[2]:
                    self.ret['error'] = 'Sytanx error (pattern of filename must have {} characters)'
                    return 'get_error', ()
            self.ret[words[1]] = words[2]
            return 'accpet', ()

    def compile(self, code) -> dict:
        lines = code.strip().split('\n')
        for line in lines:
            words = line.split()
            for i in words: i = i.strip()
            cmd, local_var = None, None
            if words[0] in self.sp_word:
                cmd, local_var = self.process_sp_word(words)