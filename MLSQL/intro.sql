

CREATE MODEL diabetes_predictor 
    USING 
        TYPE LINEAR_REGRESSION;

TRAIN diabetes_predictor
    FROM (SELECT age, heart, bpm, label from diabetes)
    USING VALIDATION_SPLIT 0.2;

SELECT * FROM new_data
    USING diabetes_predictor


