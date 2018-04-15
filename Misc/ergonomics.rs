class Token {
private:
	uint32_t value;
public:
	// Some fields omitted
	operator bool() const {
		return value;
	}
	bool operator ==(const Token &other) const {
		return value == other.value;
	}
	bool operator !=(const Token &other) const {
		return !(*this == other);
	}
};
