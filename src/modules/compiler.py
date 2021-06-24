class compiler:
    def __init__(self):
        self.sp_symbol = [('[', ']'), ('{', '}'), ('(', ')'), ('begin', 'end')]
        self.sp_word = ['set', 'int', 'float', 'list', '/', 'main', 'nftd', 'fnp', 'as', 'char']
        self.types = ('int', 'char', 'float')
        self.sp_var = ['ENDL']
        self.ret = {
            'nftd': '',
            'fnp': '',
            'error': None,
            'exec': []  # ('format_string', (types), (ranges), times)
        }
        self.flag = 0
        self.var = {}   # {'type', 'element'}

    def _is_int_(self, s) -> bool:
        try:
            int(s)
            return True
        except ValueError:
            return False

    def _is_float_(self, s) -> bool:
        try:
            float(s)
            return True
        except ValueError:
            return False

    def _is_char_(self, s) -> bool:
        try:
            chr(int(s))
            return True
        except ValueError:
            return False

    def find_quote(self, s):
        pos = {
            '[]': [],
            '{}': [],
            '()': [],
        }
        def _find_quote_(q, s):
            pre = []
            cnt = 0
            for i in range(len(s)):
                if s[i] == q[0]:
                    pre.append(i)
                    cnt += 1
                elif s[i] == q[1]:
                    if len(pre) == 0:
                        self.ret['error'] = 'Sytanx error (quote {} expect {} but exceed)'.format(q[1], cnt)
                        return 'get_error'
                    pos[q].append((pre[-1], i))
                    pre.pop()
            if not len(pre) == 0:
                self.ret['error'] = 'Sytanx error (quote {} expect {} get {})'.format(q[1], cnt, len(pre)-cnt)
                return 'get_error'
        for k in pos:
            msg = _find_quote_(k, s)
            if msg == 'get_error':
                return msg, {}
        return 'accept', pos

    def process_single_type(self, words):
        check_type = {
            'int': self._is_int_,
            'float': self._is_float_,
            'char': self._is_char_,
        }
        to_type = {
            'int': int,
            'float': float,
            'char': int,
        }
        vs = words[1:]
        var_type = words[0]
        if not var_type in self.types:
            self.ret['error'] = 'Type error (do not has type {})'.format(var_type)
            return 'get_error', ()
        for var in vs:
            var = var.strip()
            msg, pos = self.find_quote(var)
            if not msg == 'accept':
                return 'get_error', ()
            if not len(pos['[]']) == 0:
                self.ret['error'] = 'Sytanx error (did not expect quotes [])'
                return 'get_error', ()
            if len(pos['{}']) == 0:
                self.ret['error'] = 'Sytanx error (variable did not declare range)'
                return 'get_error', ()
            var_name = var[:pos['{}'][0][0]]
            if var_name in self.var:
                self.ret['error'] = 'Declaration of variable {} is ambiguous'.format(var_name)
                return 'get_error'
            rge, sit = [], 0
            for bit in range(len(pos['{}'])):
                rge.append([])
                while pos['{}'][bit][0] < pos['()'][sit][0] and pos['()'][sit][1] < pos['{}'][bit][1]:
                    sub = var[pos['()'][sit][0]+1 : pos['()'][sit][1]]
                    if not check_type[var_type](sub):
                        self.ret['error'] = 'Value error (cannot covert {} to integer)'.format(sub)
                        return 'get_error', ()
                    sub = to_type[var_type](sub)
                    if var[pos['()'][sit][0]-1] == '/':
                        if len(rge[-1]) < 2:
                            self.ret['error'] = 'Sytanx error (has not declare variable {} range)'.format(var_name)
                            return 'get_error', ()
                        else:
                            rge[-1].append(sub)
                    else:
                        rge[-1].append(sub)
                    sit += 1
                    if not sit < len(pos['()']):
                        break
            for i in range(len(rge)):
                if len(rge[i]) < 2:
                    self.ret['error'] = 'Sytanx error (variable {} did not has right range)'.format(var_name)
                    return 'get_error', ()
                rge[i] = tuple(rge[i])
            rge = tuple(rge)
            self.var[var_name] = {
                'range': rge,  # ((first, last, exceptions...), ...)
                'len': None,
                'type': var_type,
                'ele_type': None,
            }
        return 'accept', ()

    def process_list(self, words):
        check_type = {
            'int': self._is_int_,
            'float': self._is_float_,
            'char': self._is_char_,
        }
        to_type = {
            'int': int,
            'float': float,
            'char': int,
        }
        vs = words[1:]
        var_type = words[0]
        if not var_type == 'list':
            self.ret['error'] = 'Type error (do not has type {})'.format(var_type)
            return 'get_error', ()
        for var in vs:
            var = var.strip()
            msg, pos = self.find_quote(var)
            if not msg == 'accept':
                return 'get_error', ()
            if not len(pos['[]']) == 0:
                self.ret['error'] = 'Sytanx error (did not expect quotes [])'
                return 'get_error', ()
            if len(pos['{}']) == 0:
                self.ret['error'] = 'Sytanx error (variable did not declare range)'
                return 'get_error', ()
            if len(pos['()']) < 2:
                self.ret['error'] = 'Sytanx error (did not give list attribute length or element_type)'
                return 'get_error', ()
            var_name = var[:pos['()'][0][0]]
            ele_type = var[pos['()'][0][0]+1 : pos['()'][0][1]]
            length = var[pos['()'][1][0]+1 : pos['()'][1][1]]
            if not check_type['int'](length):
                self.ret['error'] = 'Type error (length of a list must be integer)'
                return 'get_error', ()
            if not ele_type in self.types:
                self.ret['error'] = 'Type error (do not has type {})'.format(var_type)
                return 'get_error', ()
            length = to_type['int'](length)
            if var_name in self.var:
                self.ret['error'] = 'Declaration of variable {} is ambiguous'.format(var_name)
                return 'get_error'
            rge, sit = [], 2
            for bit in range(len(pos['{}'])):
                rge.append([])
                while pos['{}'][bit][0] < pos['()'][sit][0] and pos['()'][sit][1] < pos['{}'][bit][1]:
                    sub = var[pos['()'][sit][0]+1 : pos['()'][sit][1]]
                    if not check_type[ele_type](sub):
                        self.ret['error'] = 'Value error (cannot covert {} to integer)'.format(sub)
                        return 'get_error', ()
                    sub = to_type[ele_type](sub)
                    if var[pos['()'][sit][0]-1] == '/':
                        if len(rge[-1]) < 2:
                            self.ret['error'] = 'Sytanx error (has not declare variable {} range)'.format(var_name)
                            return 'get_error', ()
                        else:
                            rge[-1].append(sub)
                    else:
                        rge[-1].append(sub)
                    sit += 1
                    if not sit < len(pos['()']):
                        break
            for i in range(len(rge)):
                if len(rge[i]) < 2:
                    self.ret['error'] = 'Sytanx error (variable {} did not has right range)'.format(var_name)
                    return 'get_error', ()
                rge[i] = tuple(rge[i])
            rge = tuple(rge)
            self.var[var_name] = {
                'range': rge,  # ((first, last, exceptions...), ...)
                'len': length,
                'type': var_type,
                'ele_type': ele_type,
            }
        return 'accept', ()

    def process_sp_word(self, words) -> tuple:
        if words == ('begin', 'main'):
            self.flag += 1
            return 'open_file', ()
        elif words[0] == 'begin' and words[2] == 'as':
            buf = words[1].split(',')
            for i in buf:
                i = i.strip()
                if not i in self.var:
                    self.ret['error'] = 'Variable {} is not declared in this scope'.format(i)
                    return 'get_error', ()
                if not self.var[i]['type'] == 'list':
                    self.ret['error'] = 'Object {} is not iterable'.format(self.var[i]['type'])
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
                    int(words[2])
                except ValueError:
                    self.ret['error'] = 'Value error (cannot convert {} to positive integer)'.format(words[2])
                    return 'get_error', ()
            elif words[1] == 'fnp':
                if not '{}' in words[2]:
                    self.ret['error'] = 'Sytanx error (pattern of filename must have {} characters)'
                    return 'get_error', ()
            self.ret[words[1]] = words[2]
            return 'accpet', ()
        elif words[0] in ('int', 'float', 'list', 'char'):
            if words[0] in ('int', 'float', 'char'):
                return self.process_single_type(words)
            elif words[0] == 'list':
                return self.process_list(words)
        else:
            self.ret['error'] = 'Sytanx error (unknown syntax)'
            return 'get_error', ()

    def compile(self, code) -> dict:
        lines = code.strip().split('\n')
        cnt_line = 0
        for line in lines:
            cnt_line += 1
            line = line.replace(', ', ',').replace(' ,', ',')
            words = line.split()
            for i in words: i = i.strip()
            cmd, local_var = None, None
            if words[0] in self.sp_word:
                cmd, local_var = self.process_sp_word(words)

if __name__ == '__main__':
    pass